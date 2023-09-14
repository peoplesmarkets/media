use chrono::{DateTime, Utc};
use deadpool_postgres::tokio_postgres::Row;
use deadpool_postgres::{Pool, Transaction};
use sea_query::{
    Alias, Asterisk, Expr, Iden, PostgresQueryBuilder, Query, SelectStatement,
};
use sea_query_postgres::PostgresBinder;
use uuid::Uuid;

use crate::api::peoplesmarkets::media::v1::{
    MediaFilterField, MediaOrderByField,
};
use crate::api::peoplesmarkets::ordering::v1::Direction;
use crate::db::DbError;

use super::media_offer::MediaOfferIden;
use super::MediaOfferAsRel;

#[derive(Debug, Clone, Iden)]
#[iden(rename = "medias")]
pub enum MediaIden {
    Table,
    MediaId,
    MarketBoothId,
    UserId,
    CreatedAt,
    UpdatedAt,
    Name,
    DataUrl,
}

#[derive(Debug, Clone)]
pub struct Media {
    pub media_id: Uuid,
    pub offer_ids: Option<Vec<Uuid>>,
    pub market_booth_id: Uuid,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub data_url: String,
}

impl Media {
    const OFFER_IDS_ALIAS: &str = "offer_ids";

    fn get_offer_ids_alias() -> Alias {
        Alias::new(Self::OFFER_IDS_ALIAS)
    }

    fn select_with_relations() -> SelectStatement {
        let mut query = Query::select();

        query
            .column((MediaIden::Table, Asterisk))
            .expr_as(MediaOfferAsRel::get_agg(), Self::get_offer_ids_alias())
            .from(MediaIden::Table)
            .left_join(
                MediaOfferIden::Table,
                Expr::col((MediaIden::Table, MediaIden::MediaId))
                    .equals((MediaOfferIden::Table, MediaOfferIden::MediaId)),
            )
            .group_by_col((MediaIden::Table, MediaIden::MediaId));

        query
    }

    pub async fn create<'a>(
        transaction: &Transaction<'a>,
        media_id: &Uuid,
        market_booth_id: &Uuid,
        user_id: &String,
        name: &String,
        file_path: &String,
    ) -> Result<Self, DbError> {
        let (sql, values) = Query::insert()
            .into_table(MediaIden::Table)
            .columns([
                MediaIden::MediaId,
                MediaIden::MarketBoothId,
                MediaIden::UserId,
                MediaIden::Name,
                MediaIden::DataUrl,
            ])
            .values([
                (*media_id).into(),
                (*market_booth_id).into(),
                user_id.into(),
                name.into(),
                file_path.into(),
            ])?
            .returning_all()
            .build_postgres(PostgresQueryBuilder);

        let row = transaction
            .query_one(sql.as_str(), &values.as_params())
            .await?;

        Ok(Self::from(row))
    }

    pub async fn get(
        pool: &Pool,
        media_id: &Uuid,
    ) -> Result<Option<Self>, DbError> {
        let client = pool.get().await?;

        let (sql, values) = Self::select_with_relations()
            .and_where(
                Expr::col((MediaIden::Table, MediaIden::MediaId)).eq(*media_id),
            )
            .build_postgres(PostgresQueryBuilder);

        let row = client.query_opt(sql.as_str(), &values.as_params()).await?;

        Ok(row.map(Self::from))
    }

    pub async fn list(
        pool: &Pool,
        market_booth_id: &Uuid,
        user_id: &String,
        limit: u64,
        offset: u64,
        _filter: Option<(MediaFilterField, String)>,
        _order_by: Option<(MediaOrderByField, Direction)>,
    ) -> Result<Vec<Self>, DbError> {
        let client = pool.get().await?;

        let (sql, values) = {
            let mut query = Self::select_with_relations();

            query
                .and_where(
                    Expr::col((MediaIden::Table, MediaIden::MarketBoothId))
                        .eq(*market_booth_id),
                )
                .and_where(
                    Expr::col((MediaIden::Table, MediaIden::UserId))
                        .eq(user_id),
                )
                .limit(limit)
                .offset(offset)
                .build_postgres(PostgresQueryBuilder)
        };

        let rows = client.query(sql.as_str(), &values.as_params()).await?;

        Ok(rows.iter().map(Self::from).collect())
    }

    pub async fn update(
        pool: &Pool,
        media_id: &Uuid,
        user_id: &String,
        name: Option<String>,
    ) -> Result<Self, DbError> {
        let client = pool.get().await?;

        let (sql, values) = {
            let mut query = Query::update();
            query.table(MediaIden::Table);

            if let Some(name) = name {
                query.value(MediaIden::Name, name);
            }

            query
                .and_where(Expr::col(MediaIden::MediaId).eq(*media_id))
                .and_where(Expr::col(MediaIden::UserId).eq(user_id))
                .returning_all()
                .build_postgres(PostgresQueryBuilder)
        };

        let row = client.query_one(sql.as_str(), &values.as_params()).await?;

        Ok(Self::from(row))
    }

    pub async fn begin_delete<'a>(
        transaction: &Transaction<'a>,
        media_id: &Uuid,
        user_id: &String,
    ) -> Result<(), DbError> {
        let (sql, values) = Query::delete()
            .from_table(MediaIden::Table)
            .and_where(Expr::col(MediaIden::MediaId).eq(*media_id))
            .and_where(Expr::col(MediaIden::UserId).eq(user_id))
            .build_postgres(PostgresQueryBuilder);

        transaction
            .execute(sql.as_str(), &values.as_params())
            .await?;

        Ok(())
    }
}

impl From<&Row> for Media {
    fn from(row: &Row) -> Self {
        Self {
            media_id: row.get(MediaIden::MediaId.to_string().as_str()),
            offer_ids: row.try_get(Self::OFFER_IDS_ALIAS).ok(),
            market_booth_id: row
                .get(MediaIden::MarketBoothId.to_string().as_str()),
            user_id: row.get(MediaIden::UserId.to_string().as_str()),
            created_at: row.get(MediaIden::CreatedAt.to_string().as_str()),
            updated_at: row.get(MediaIden::UpdatedAt.to_string().as_str()),
            name: row.get(MediaIden::Name.to_string().as_str()),
            data_url: row.get(MediaIden::DataUrl.to_string().as_str()),
        }
    }
}

impl From<Row> for Media {
    fn from(row: Row) -> Self {
        Self::from(&row)
    }
}
