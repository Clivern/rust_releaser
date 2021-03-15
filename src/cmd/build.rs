// Copyright 2021 Clivern. All rights reserved.
// Use of this source code is governed by the MIT
// license that can be found in the LICENSE file.

use log;

// build command
pub fn build(config: &str) -> Result<String, String> {
    log::info!("Start build command handler with config file: {}", config);

    Ok(String::from(""))
}
