use mongodb::{Collection, Database, bson::{doc, self}};
use tide::{Request, Response, http::StatusCode, Body};
use serde::{Serialize, Deserialize};
// use mongodb::bson;
use bson::oid::ObjectId;
use futures::stream::StreamExt;

/**
Struct Untuk Function Insert
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Mahasiswa {
    // #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    // pub id: Option<ObjectId>,
    // #[model(index(index = "dsc", with(field = "mahasiswa", index = "dsc")))]
    pub nim: String,
    pub nama: String,
    pub hobi: String
}

/**
Struct Untuk Function List \
Nevemind
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct MahasiswaL {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    // #[model(index(index = "dsc", with(field = "mahasiswa", index = "dsc")))]
    pub nim: String,
    pub nama: String,
    pub hobi: String
}

/**
Struct Untuk Function Edit
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct MahasiswaE {
    // #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    // #[model(index(index = "dsc", with(field = "mahasiswa", index = "dsc")))]
    pub id: String,
    pub nim: String,
    pub nama: String,
    pub hobi: String
}

/**
Struct Untuk Function Delete
*/
#[derive(Debug, Deserialize)]
pub struct PKM {
    pub id: String
}

/**
List Semua Dokumen dalam Collection Mahasiswa
*/
pub async fn list(req: Request<Database>) -> tide::Result<Response> {
    let state = req.state();
    // let coll = state.collection("mahasiswa");
    let coll: Collection<MahasiswaL> = state.collection_with_type("mahasiswa");

    // let filter = doc! { "nama": "Testing 1" };
    // let options = FindOptions::builder().sort(doc! { "nim": 1 }).build();
    let mut cursor = coll.find(None, None).await?;
    let mut docs = Vec::new();

    while let Some(doc) = cursor.next().await {
        docs.push(doc?);
    }

    // Read the document from a MongoDB collection
    // let person_document = mongoCollection.find_one(Some(doc! { "_id" => "12345" }), None)?
    // .expect("Document not found");

    // Deserialize the document into a Person instance
    // let person = bson::from_bson(bson::Bson::Document(person_document))?;

    let mut resp = Response::new(StatusCode::Ok);
    let json = serde_json::json!({
        "status": "OK",
        "info": "Data Berhasil",
        "data": &docs
    });
    resp.set_body(Body::from_json(&json)?);
    Ok(resp)
}

/**
 Insert Satu Data ke dalam Collection Mahasiswa
*/
pub async fn insert(mut req: Request<Database>) -> tide::Result<Response> {
    let request: Mahasiswa = req.body_json().await?;
    let state = req.state();
    let coll = state.collection("mahasiswa");
    let serialized_post = bson::to_bson(&request)?;
    // let from = bson::from_bson(serialized_post)?;
    println!("Serialized: {:?}", serialized_post);
    let _insert = coll.insert_one(bson::from_bson(serialized_post)?, None).await?;
    // How to insert many with Vec<Struct> ?

    let mut resp = Response::new(StatusCode::Ok);
    let json = serde_json::json!({
        "status": "OK",
        "info": "Data Berhasil di Simpan"
    });
    resp.set_body(Body::from_json(&json)?);
    Ok(resp)
}

/**
Insert Banyak Data dalam satu API ke Collection Mahasiswa
*/
pub async fn insert_banyak(mut req: Request<Database>) -> tide::Result<Response> {
    let request: Vec<Mahasiswa> = req.body_json().await?;
    let state = req.state();
    let coll = state.collection("mahasiswa");
    // How to insert many with Vec<Struct> ?
    // let _insert = coll.insert_many(request, None).await?;
    for data in request {
        let serialized_post = bson::to_bson(&data)?;
        let _insert = coll.insert_one(bson::from_bson(serialized_post)?, None).await?;
    }

    let mut resp = Response::new(StatusCode::Ok);
    let json = serde_json::json!({
        "status": "OK",
        "info": "Data Berhasil di Simpan"
    });
    resp.set_body(Body::from_json(&json)?);
    Ok(resp)
}

/**
Edit Dokumen Mahasiswa dengan Parameter _id
*/
pub async fn edit(mut req: Request<Database>) -> tide::Result<Response> {
    let data: MahasiswaE = req.body_json().await?;
    let state = req.state();
    let coll = state.collection("mahasiswa");

    // update dengan id document
    let object_id = ObjectId::with_string(&data.id)?;
    let filter = doc! { "_id": object_id };

    // let query = doc! { "nim": data.nim };
    let update = doc! { 
        "$set": { "nim": data.nim, "nama": data.nama, "hobi": data.hobi },
        "$currentDate": { "lastModified": true }
    };

    let _upd = coll.update_one(filter, update, None).await?;

    let mut resp = Response::new(StatusCode::Ok);
    let json = serde_json::json!({
        "status": "OK",
        "info": "Data Berhasil di Edit"
    });
    resp.set_body(Body::from_json(&json)?);
    Ok(resp)
}

/**
Delete Dokumen Mahasiswa dengan Query String id
*/
pub async fn delete(req: Request<Database>) -> tide::Result<Response> {
    let state = req.state();
    let query: PKM = req.query()?;
    let coll = state.collection("mahasiswa");

    let object_id = ObjectId::with_string(&query.id)?;
    let query = doc! {"_id": object_id};
    let _delete = coll.delete_one(query, None).await?;

    let mut resp = Response::new(StatusCode::Ok);
    let json = serde_json::json!({
        "status": "OK",
        "info": "Data Berhasil di Hapus"
    });
    resp.set_body(Body::from_json(&json)?);
    Ok(resp)
}