extern crate av_data;
extern crate av_format;
extern crate matroska;
extern crate nom;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use av_format::buffer::AccReader;
use av_format::demuxer::{self, Event};
use av_format::muxer;
use std::fs::File;
use std::io::Cursor;
use std::sync::Arc;

use matroska::demuxer::MKV_DESC;
use matroska::muxer::MkvMuxer;

fn main() {
    pretty_env_logger::init();
    //const webm: &'static [u8] = include_bytes!("../assets/big-buck-bunny_trailer.webm");
    const WEBM: &'static [u8] = include_bytes!("../assets/bbb-vp9-opus.webm");
    //const webm: &'static [u8] = include_bytes!("../assets/single_stream.mkv");
    let d = MKV_DESC.create();
    let c = Cursor::new(WEBM);
    //let acc = AccReader::with_capacity(5242880, c);
    let acc = AccReader::with_capacity(20000, c);
    let input = Box::new(acc);
    let mut demuxer = demuxer::Context::new(d, input);

    println!("read headers: {:?}", demuxer.read_headers().unwrap());
    println!("global info: {:#?}", demuxer.info);

    let mux = Box::new(MkvMuxer::webm());
    let output = File::create("output.webm").unwrap();

    let mut muxer = muxer::Context::new(mux, Box::new(output));
    muxer.configure().unwrap();
    muxer.set_global_info(demuxer.info.clone()).unwrap();
    muxer.write_header().unwrap();

    loop {
        match demuxer.read_event() {
            Ok(event) => {
                //println!("event: {:?}", event);
                match event {
                    Event::MoreDataNeeded(sz) => panic!("we needed more data: {} bytes", sz),
                    Event::NewStream(s) => panic!("new stream :{:?}", s),
                    Event::NewPacket(packet) => {
                        //println!("writing packet");
                        muxer.write_packet(Arc::new(packet)).unwrap();
                    }
                    Event::Eof => {
                        muxer.write_trailer().unwrap();
                        break;
                    }
                }
            }
            Err(e) => {
                println!("error: {:?}", e);
                break;
            }
        }
    }
}
