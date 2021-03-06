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

extern crate serde;

use ::resource::config::ConfigLoader;
use ::resource::plugin::PluginLoader;
use ::util::Directory;

use self::serde::{Deserialize, Serialize};

use std::cell::RefCell;
use std::rc::Rc;

/*===============================================================================================*/
/*------RESOURCE MANAGER STRUCT------------------------------------------------------------------*/
/*===============================================================================================*/

/// Interface for resource loading and management.
pub struct ResourceManager {

    // Private
    _config_loader: Rc<RefCell<ConfigLoader>>,
    _plugin_loader: Rc<RefCell<PluginLoader>>,
}

/*===============================================================================================*/
/*------RESOURCE MANAGER PUBLIC METHODS----------------------------------------------------------*/
/*===============================================================================================*/

impl ResourceManager {

    /// Initializes the Resource Manager.
    pub fn init (&mut self) {

        info! ("Initializing the Resource Manager.");
        self._config_loader.borrow ().init ();
        self._plugin_loader.borrow_mut ().init (self);
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Creates a new config file.
    pub fn new_config<T: Default + Serialize> (&self, config_name: &str) -> Result<(), ()> {

        let p_config_dir = &Directory::get_persistent_config_directory ();
        self._config_loader.borrow ().new_config::<T> (p_config_dir, config_name)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Loads a config file.
    pub fn load_config<T: Deserialize> (&self, config_name: &str) -> Result<T, ()> {

        let p_config_dir = &Directory::get_persistent_config_directory ();
        self._config_loader.borrow ().load_config::<T> (p_config_dir, config_name)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Saves a config file.
    pub fn save_config<T: Serialize> (&self, config_name: &str, config_data: &T) -> Result<(), ()> {

        let p_config_dir = &Directory::get_persistent_config_directory ();
        self._config_loader.borrow ().save_config::<T> (p_config_dir, config_name, config_data)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a pointer to the config loader instance.
    pub unsafe fn get_config_loader_raw (&self) -> Rc<RefCell<ConfigLoader>> {
        self._config_loader.clone ()
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a pointer to the plugin loader instance.
    pub unsafe fn get_plugin_loader_raw (&self) -> Rc<RefCell<PluginLoader>> {
        self._plugin_loader.clone ()
    }

/*===============================================================================================*/
/*------RESOURCE MANAGER PUBLIC STATIC METHODS---------------------------------------------------*/
/*===============================================================================================*/

    /// Create a new instance of the Resource Manager.
    pub fn new () -> ResourceManager {

        ResourceManager {

            _config_loader: Rc::new (RefCell::new (ConfigLoader {})),
            _plugin_loader: Rc::new (RefCell::new (PluginLoader::new ()))
        }
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Default for ResourceManager {

    fn default () -> ResourceManager {
        ResourceManager::new ()
    }
}
