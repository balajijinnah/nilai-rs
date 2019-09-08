/*
 * Copyright 2019 balajijinnah and Contributors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use failure::Error;
use nilai::builder;
use nilai::types;
use std::thread;
use std::time::Duration;

fn do_main() -> Result<(), Error> {
    let nilai_builder = builder::NilaiBuilder::new("127.0.0.1:5001".parse()?);
    let closer = nilai_builder
        .alive_delegate(Box::new(|_: types::Node| println!("new node joined")))
        .execute()?;
    // nilai is running so block the current thread.
    thread::sleep(Duration::from_secs(5));
    closer.stop();
    Ok(())
}

fn main() {
    match do_main() {
        Err(err) => {
            println!("not able to run nilai handler {:?}", err);
        }
        _ => {}
    }
}
