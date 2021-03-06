/*===============================================================================================*/
// Copyright 2016 Kyle Finlay
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
/*===============================================================================================*/

/*===============================================================================================*/
/*------QUAT STRUCT------------------------------------------------------------------------------*/
/*===============================================================================================*/

/// Quaternion implementation.
///
/// Allows for representing rotations without gimbal lock.
#[derive (Copy, Clone, Default, Serialize, Deserialize)]
pub struct Quat {

    // Public
    /// X-axis coordinate.
    pub x: f32,
    /// Y-axis coordinate.
    pub y: f32,
    /// Z-axis coordinate.
    pub z: f32,
    /// W-axis coordinate.
    pub w: f32,
}
