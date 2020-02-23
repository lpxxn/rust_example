use sqlx;
use async_std::task;
pub fn sqlx_lib_fn() {
    let pool = task::block_on(sqlx::PgPool::new("postgres://:@127.0.0.1:5432/baseinfo_test?sslmode=disable")).unwrap();
    println!("{:?}", pool);    
    // if let Ok(v) = pool {
    //     println!("{:?}", v);
    // } else {
    // }
   
//    let rev = async {
//        sqlx::query("SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'").fetch(&mut pool).map_ok(|row| row.name("table_name")).try_collect().await.unwrap();
//    };
    //let rev = task::block_on(sqlx::query("SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'").fetch(&mut &pool).map_ok(|row|row.name("table_name")).try_collect()).unwrap();
}