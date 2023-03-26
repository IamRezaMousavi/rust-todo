/**
 * @Author: @iamrezamousavi
 * @Date:   2023-03-24 02:34:45
 * @Last Modified by:   @iamrezamousavi
 * @Last Modified time: 2023-03-26 05:27:07
 */
pub mod db;
pub mod server;

use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct Message {
    message: String,
}
