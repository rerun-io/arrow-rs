// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

fn main() -> Result<(), Box<dyn std::error::Error>> {

    //println!("cargo:rerun-if-changed=../format/Flight.proto");

    tonic_build::configure()
        // protoc in unbuntu builder needs this option
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(&["../format/Flight.proto"], &["../format"])?;
    
    tonic_build::configure()
        // protoc in unbuntu builder needs this option
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(&["../format/FlightSql.proto"], &["../format"])?;

    // Prost currently generates an empty file, this was fixed but then reverted
    // https://github.com/tokio-rs/prost/pull/639
    //let google_protobuf_rs = Path::new("src/sql/google.protobuf.rs");
    //if google_protobuf_rs.exists() && google_protobuf_rs.metadata().unwrap().len() == 0 {
    //    std::fs::remove_file(google_protobuf_rs).unwrap();
    //}

    // As the proto file is checked in, the build should not fail if the file is not found
    Ok(())
}
