mod media;

pub use media::MediaService;

use tonic::Status;
use uuid::Uuid;

use crate::api::peoplesmarkets::pagination::v1::Pagination;

fn uuid_err_to_grpc_status(field: &str) -> Status {
    Status::invalid_argument(format!("field {field} is not a valid UUID v4"))
}

fn parse_uuid(uuid_string: &str, field: &str) -> Result<Uuid, Status> {
    uuid_string
        .parse()
        .map_err(|_| uuid_err_to_grpc_status(field))
}

/**
 * Returns limit and offset for requested Pagination or defaults.
 */
fn paginate(
    request: Option<Pagination>,
) -> Result<(u64, u64, Pagination), Status> {
    let mut limit = 10;
    let mut offset = 0;
    let mut pagination = Pagination {
        page: 1,
        size: limit,
    };

    if let Some(request) = request {
        if request.page < 1 {
            return Err(Status::invalid_argument("pagination.page"));
        }
        limit = request.size;
        offset = (request.page - 1) * request.size;
        pagination = request;
    }

    Ok((limit, offset, pagination))
}
