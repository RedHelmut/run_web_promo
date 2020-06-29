#![feature(proc_macro_hygiene, decl_macro)]
//[dependencies.promo_fin]
// path = "F:/rustprojects/promo_fin"

use std::{thread, env, io};
use std::time::{Instant, Duration};
use std::fs::File;
use std::io::{Read, BufReader, ErrorKind, Write};
use crossbeam::channel::{Receiver, Sender};

use promo_fin::missing_report::run_missing_reports;
use promo_fin::pdf::write_rows_to_pdf_container;
use std::path::{Path, PathBuf};
use router::Router;
use multipart::server::nickel::nickel::{Mount, StaticFilesHandler};
use iron::{IronResult, Response, status, Request, Iron, Set};
use multipart::server::{SaveResult, Multipart, Entries};
use std::collections::HashMap;
use std::ops::Deref;
use staticfile::Static;

#[macro_use]
extern crate mime;
/*
fn upload(data: Data) -> Option<NamedFile> {//Result<String, Debug<io::Error>> {
    let mut options = MultipartFormDataOptions::new();
/*    options.allowed_fields.push(
        MultipartFormDataField::raw("avatar")
            .size_limit(8 * 1024 * 1024) // 8 MB
            .content_type_by_string(Some(mime::IMAGE_STAR))
            .unwrap(),
    );*/

    options
        .allowed_fields
        .push(MultipartFormDataField::file("data").content_type(Some(mime::STAR_STAR)));

    let mut multipart_form_data = MultipartFormData::parse(&ContentType::FormData, data, options).unwrap();
    return None;
    println!("Ok");
    let path = env::temp_dir().join("upload.csv");
    let pc = path.clone();
    /* let p = data.stream_to_file(path)
         .map(|n| {

             n.to_string()
         })
         .map_err(Debug);
 */
    let file = pc.to_str().unwrap();
    let json = r##"F:\3M Promo Data\May1-July31 2020\promo_May1_2020-July31_2020.json"##;

    if let Some(reo) = env::temp_dir().join("promos.zip").to_str() {
        println!("File Name: {}", file);
        match run_missing_reports::<File>(file, json, None, reo) {
            Ok(k) => {
                Some(NamedFile::open(reo).unwrap())
            }
            Err(e) => {
                println!("Error {}", e.to_string());
                None
            }
        }
    } else {
        None
    }

}
*/

fn get_form(_request: &mut iron::Request) -> IronResult <iron::Response> {
    let mut response = iron::Response::new();
    response.set_mut( iron::status::Ok);
   // let mi: mime::Mime = "Text/Html;Charset=Utf8".parse().unwrap();
//    response.set_mut( mime::TEXT_PLAIN_UTF_8);
    response.headers.set(iron::headers::ContentType("text/html; charset=utf-8".parse::<iron::mime::Mime>().unwrap()));
    response.set_mut(
        r##"
            <!DOCTYPE html>
            <html lang="en">
            <head>
            <title>CSS Template</title>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <style>
            * {
            box-sizing: border-box;
            }

            body {
            font-family: Arial, Helvetica, sans-serif;
            }

            /* Style the header */
            header {
            background-color: #666;
            padding: 30px;
            text-align: center;
            font-size: 35px;
            color: white;
            }

            /* Create two columns/boxes that floats next to each other */
            nav {
            float: left;
            width: 40%;
            background: #ccc;
            padding: 20px;
            }

            dav {
            float: left;
            width: 40%;
            background: #ccc;
            padding: 20px;
            }


            /* Style the list inside the menu */
            nav ul {
            list-style-type: none;
            padding: 0;
            }

            article {
            float: left;
            padding: 20px;
            width: 70%;
            background-color: #f1f1f1;
            height: 300px; /* only for demonstration, should be removed */
            }

            /* Clear floats after the columns */
            section:after {
            content: "";
            display: table;
            clear: both;
            }

            /* Style the footer */
            footer {
            background-color: #777;
            padding: 10px;
            text-align: center;
            color: white;
            }
            dooter {
            background-color: #777;
            text-align: center;
            color: white;
            }
            /* Responsive layout - makes the two columns/boxes stack on top of each other instead of next to each other, on small screens */
            @media (max-width: 600px) {
            nav, article {
                width: 100%;
                height: auto;
            }
            }
            </style>
            </head>
            <body>
                <form action="/3mInfo" method="post" enctype="multipart/form-data" >

                    <h2>Please upload both files</h2>
                    <img src="andreaproposal.jpg"/>
                    <section>
                        <nav>
                        </br><dooter><input type = "file" name = "uploaded_promo_file" accept=".json" multiple="false" >Promo File .json</input></dooter>
                        </br><dooter><input type = "file" name = "uploaded_csv_file" accept=".csv" multiple="false">Customer Data .csv</input></dooter>
                        </br>
                        </nav>

                    </section>
                    <dav>
                        <input type = "submit"/>
                    </dav>
                </form>
            </body>
            </html>
        "##
    );
    Ok( response) }

fn process_entries(entries: Entries) -> IronResult<iron::Response> {
    let pth = &entries.save_dir.into_path();

    let mut files: HashMap<String, String> = HashMap::new();
    for files_in_multipart in &entries.fields {
        let to_be_the_name_of_file = files_in_multipart.0.clone();

        let temp_file_path_dir = pth.to_str().unwrap().to_owned();

        for saved_field in files_in_multipart.1 {
            if saved_field.headers.filename.clone().unwrap().trim() == "" {
                continue;
            }
            use multipart::server::save::SavedData::*;

            match &saved_field.data {
                Text(text) => {},
                Bytes(bytes) => {
                    //not in file format so write to file
                    match to_be_the_name_of_file.as_ref() {
                        x if x == "uploaded_promo_file" || x == "uploaded_csv_file" => {
                            let mut add_ext = None;
                            if let Some( name ) = &saved_field.headers.filename {
                                let path = Path::new( name );
                                if let Some(ext) = path.extension() {
                                    if let Some(real_ext) = ext.to_str() {
                                        add_ext = Some(real_ext);
                                    }
                                }
                            }
                            let file_p = match add_ext {
                                Some( ext ) => {
                                    format!("{}/temp_{}.{}", temp_file_path_dir, x.to_owned(), ext)
                                }, None => {
                                    format!("{}/temp_{}", temp_file_path_dir, x.to_owned())
                                }
                            };

                            let created_file = std::fs::File::create(file_p.clone());
                            match created_file {
                                Ok(mut file_c) => {
                                    match file_c.write_all(bytes) {
                                        Ok(()) => {
                                            if let Err(fail_err) = file_c.flush() {
                                                let desc = fail_err.to_string();
                                                return Err(iron::IronError { error: Box::new(fail_err), response: iron::Response::with((iron::status::Ok, format!("Error: {} ", desc))) });
                                            }
                                            files.entry(x.to_owned()).or_insert(file_p.clone());
                                        },
                                        Err(errz) => {
                                            let desc = errz.to_string();
                                            return Err(iron::IronError { error: Box::new(errz), response: iron::Response::with((iron::status::Ok, format!("Error: {} ", desc))) });
                                        }
                                    }
                                },
                                Err(err) => {
                                    let desc = err.to_string();
                                    return Err(iron::IronError { error: Box::new(err), response: iron::Response::with((iron::status::Ok, format!("Error: {} ", desc))) });
                                },
                            }
                        },
                        _ => {//if no error just ignore this case, building a hashmap
                            ()
                        }
                    };
                },
                File(file, _size) => {
                    match to_be_the_name_of_file.as_ref() {
                        x if x == "uploaded_promo_file" || x == "uploaded_csv_file" => {
                            let path_buffer = file.to_path_buf();
                            let file_path = file.to_path_buf().to_str().unwrap().to_owned();

                            let mut add_ext = None;
                            if let Some( name ) = &saved_field.headers.filename {
                                let path = Path::new( name );
                                if let Some(ext) = path.extension() {
                                    if let Some(real_ext) = ext.to_str() {
                                        add_ext = Some(real_ext);
                                    }
                                }
                            }
                            let file_p = match add_ext {
                                Some( ext ) => {
                                    let new_path = format!("{}.{}", file_path, ext);
                                    std::fs::rename(file_path, new_path.clone());
                                    new_path
                                }, None => {
                                    format!("{}", file_path)
                                }
                            };


                            files.entry(x.to_owned()).or_insert(format!("{}", file_p));
                            ()
                        },
                        _ => {//if no error just ignore this case, building a hashmap
                            ()
                        }
                    }
                }
            }
        }
    }

    match (files.get::<str>(&"uploaded_csv_file"), files.get::<str>(&"uploaded_promo_file") ) {
        (Some( csv),Some(json )) => {
            let zip_file_path_opt = env::temp_dir().join("Promo.zip");
            match zip_file_path_opt.to_str() {
                Some( zip_path ) => {

                    match run_missing_reports::<File>(csv, json, None, zip_path) {
                        Ok(k) => {
                            let pth = PathBuf::from(zip_path);
                            Ok(iron::Response::with((iron::status::Ok, pth)))
                        }
                        Err(e) => {
                            println!("Error {}", e.to_string());
                            Ok(iron::Response::with((iron::status::Ok, "Failure")))
                        }
                    }
                },
                None => {
                    Ok(iron::Response::with((iron::status::Ok, "Server Error")))
                }
            }

        },
        (None, None) => {
            Ok(iron::Response::with((iron::status::Ok, "")))
        },
        (Some(_), None) => {
            Ok(iron::Response::with((iron::status::Ok, "")))
        },
        (None,Some(_)) => {
            Ok(iron::Response::with((iron::status::Ok, "")))
        }
    }


}
fn process_request(request: &mut Request) -> IronResult<Response> {
    // Getting a multipart reader wrapper

    match Multipart::from_request(request) {
        Ok(mut multipart) => {
            // Fetching all data and processing it.
            // save().temp() reads the request fully, parsing all fields and saving all files
            // in a new temporary directory under the OS temporary directory.
            match multipart.save().temp() {
                SaveResult::Full(entries) => process_entries(entries),
                SaveResult::Partial(entries, reason) => {
                    process_entries(entries.keep_partial())?;
                    Ok(Response::with((
                        status::BadRequest,
                        format!("error reading request: {}", reason.unwrap_err())
                    )))
                }
                SaveResult::Error(error) => Ok(Response::with((
                    status::BadRequest,
                    format!("error reading request: {}", error)
                ))),
            }
        }
        Err(_) => {
            Ok(Response::with((status::BadRequest, "The request is not multipart")))
        }
    }
}

fn main() {
    let mut router = Router::new();
    router.get("/", get_form, "root");
    router.post("/3mInfo", process_request, "gcd");
   // let mut assets_mount = Mount::new("/static_files/",
  //                                    StaticFilesHandler::new("/path/to/serve/");
 /*   assets_mount
        .mount("/", router )
        .mount("/assets/", Static::new(Path::new("src/assets")));*/

    Iron::new(router).http("127.0.0.1:8080").unwrap();

}