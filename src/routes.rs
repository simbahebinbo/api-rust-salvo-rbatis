
use salvo::{hyper::Method, prelude::*};
use crate::database::{Blog, RB};

#[handler]
pub async fn blogs(req: &mut Request, res: &mut Response) {
    match req.method() {
        &Method::GET => match req.query::<i32>("id") {
            Some(id) => {
                let blog = Blog::select_by_column(&mut RB.clone(), "id", id)
                    .await
                    .unwrap();
                res.render(Json(blog));
            }
            _ => {
                let blog_vector = Blog::select_all(&mut RB.clone()).await.unwrap();
                res.render(Json(blog_vector));
            }
        },
        &Method::POST => {
                let blog_data = req.parse_body::<Blog>().await.unwrap();
                let blog = Blog::new(blog_data.title, blog_data.description);
                Blog::insert(&mut RB.clone(), &blog).await.unwrap();

                res.render(Json(blog));
        }
        &Method::DELETE => {
            let blog_id = req.query::<i32>("id").unwrap();
            Blog::delete_in_column(&mut RB.clone(), "id", &[blog_id.to_string()])
                .await
                .unwrap();
            res.render(Text::Html("The text was deleted successfully"));
        }
        &Method::PUT => {
            let blog_id = req.query::<i32>("id").unwrap();
            let blog = req.parse_json::<Blog>().await.unwrap();

            Blog::update_by_column_value(&mut RB.clone(), &blog, "id", &rbs::Value::I32(blog_id))
                .await
                .unwrap();
            res.render(Text::Html("The text was updated successfully"));
        }
        _ => {
            res.set_status_code(StatusCode::BAD_REQUEST);
            res.render(Text::Html("the method is not valid"));
        }
    }
}
