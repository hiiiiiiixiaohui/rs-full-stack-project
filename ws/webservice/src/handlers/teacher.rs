use crate::dbaccess::teacher::*;
use crate::errors::SysError;
use crate::models::teacher::{CreateTeacher, UpdateTeacher};
use crate::state::AppState;

use actix_web::{web, HttpResponse};

pub async fn get_all_teachers(app_state: web::Data<AppState>) -> Result<HttpResponse, SysError> {
    get_all_teachers_db(&app_state.db)
        .await
        .map(|tc| HttpResponse::Ok().json(tc))
}

pub async fn get_teacher_details(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, SysError> {
    let tearcher_id = params.into_inner();
    get_teacher_details_db(&app_state.db, tearcher_id)
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn post_new_teacher(
    app_state: web::Data<AppState>,
    new_teacher: web::Json<CreateTeacher>,
) -> Result<HttpResponse, SysError> {
    post_new_teacher_db(&app_state.db, CreateTeacher::from(new_teacher))
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn update_teacher_details(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
    update_teacher: web::Json<UpdateTeacher>,
) -> Result<HttpResponse, SysError> {
    let teacher_id = params.into_inner();
    update_teacher_details_db(
        &app_state.db,
        teacher_id,
        UpdateTeacher::from(update_teacher),
    )
    .await
    .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn delete_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, SysError> {
    let teacher_id = params.into_inner();

    delete_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
}

#[cfg(test)]

mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::ResponseError;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use std::sync::Mutex;

    // test get all teachers
    #[ignore = "tested"]
    #[actix_rt::test]
    async fn get_all_teachers_success_ts() {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let resp = get_all_teachers(app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[ignore = "tested"]
    #[actix_rt::test]
    async fn get_one_teacher_detail_success_ts() {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let teacher_id = web::Path::from(123);

        let resp = get_teacher_details(app_state, teacher_id).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[ignore = "tested"]
    #[actix_rt::test]
    async fn post_new_teacher_success_ts() {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let new_teacher = web::Json(CreateTeacher {
            name: "new teacher".into(),
            picture_url: "https://xx".into(),
            profile: "hello, i am new teacher".into(),
        });

        let resp = post_new_teacher(app_state, new_teacher).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[ignore = "tested"]
    #[actix_rt::test]
    async fn update_teacher_details_success_ts() {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let update_teacher = web::Json(UpdateTeacher {
            name: Some("changed new teacher".into()),
            picture_url: Some("https://xx".into()),
            profile: Some("hello, i am changed new teacher".into()),
        });

        let params: web::Path<i32> = web::Path::from(1);

        let resp = update_teacher_details(app_state, params, update_teacher)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[ignore = "tested"]
    #[actix_rt::test]
    async fn delete_teacher_success_ts() {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<i32> = web::Path::from(3);

        let resp = delete_teacher(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[ignore = "tested"]
    #[actix_rt::test]
    async fn delete_teacher_failure_ts() {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<i32> = web::Path::from(100);

        let resp = delete_teacher(app_state, params).await;

        match resp {
            Ok(_) => println!("Something wrong..."),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }
}
