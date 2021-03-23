use tide::Server;
use mongodb::Database;
use crate::handler;

pub fn set(app: &mut Server<Database>) -> Result<(), std::io::Error> {

    // PEMBAHASAN 01 : list record, output ke json
    // app.at("/kantor")
    //     .get(handler::kantor::list)
    //     .post(handler::kantor::tambah);

    app.at("/mahasiswa")
        .get(handler::mahasiswa::list)
        .post(handler::mahasiswa::insert)
        .patch(handler::mahasiswa::edit)
        .delete(handler::mahasiswa::delete);
    
    app.at("/mahasiswa/banyak").post(handler::mahasiswa::insert_banyak);

    Ok(())
}