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

use ::resource::ResourceManager;
use ::window::WindowManager;
use ::util::{Version, Logger};

use std::cell::RefCell;
use std::env;
use std::rc::Rc;
use std::boxed::Box;
use std::path::Path;
use std::fs;
use std::process;

/*===============================================================================================*/
/*------STATIC VARIABLES-------------------------------------------------------------------------*/
/*===============================================================================================*/

static mut APP_POINTER: Option <*mut App> = None;

/*===============================================================================================*/
/*------APP STRUCT-------------------------------------------------------------------------------*/
/*===============================================================================================*/

/// The app
///
/// This is the main control center of ionCore.
/// It is in charge of initialization, updating, and shutdown of all modules,
/// as well as the handing of any inter-module communication.
pub struct App {

    // Public
    /// The resource manager.
    pub resource_mgr: Rc<RefCell<ResourceManager>>,
    /// The window manager.
    pub window_mgr: Rc<RefCell<WindowManager>>,

    /// The project name.
    pub project_name: String,
    /// The project developer,
    pub project_developer: String,
    /// The project version.
    pub project_version: Version,

    /// The resource directory.
    pub res_dir: String,
    /// The binary directory.
    pub bin_dir: String,
    /// The plugin directory.
    pub plg_dir: String,
    /// The config directory.
    pub cfg_dir: String,

    // Private
    _is_in_main_loop: bool,
    _should_exit: bool,
}

/*===============================================================================================*/
/*------APP PUBLIC METHODS-----------------------------------------------------------------------*/
/*===============================================================================================*/

impl App {

    /// Initializes the app
    pub fn init (&self) {

        self._check_dirs_for_errors ();

        Logger::init (&format! ("{}ionEngine.log", &self.cfg_dir), true).unwrap ();
        info! ("Initializing ionCore | Version: {}", env! ("CARGO_PKG_VERSION"));

        // Init the managers
        self._init_managers ();
    }

/*-----------------------------------------------------------------------------------------------*/

    /// The main app loop.
    pub fn run (&mut self) {

        self._is_in_main_loop = true;

        loop {

            let should_exit = self._should_exit;

            if !should_exit {

                self._on_pre_render ();
                self._on_render ();
                self._on_post_render ();
            }

            else {

                self._is_in_main_loop = false;
                return;
            }
        }
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Releases all resources, and exits the application.
    pub fn exit (&mut self) {

        // Check if in main loop
        if self._is_in_main_loop {

            info! ("Exiting main loop.");
            self._should_exit = true;
        }

        else {

            info! ("Shutting down ion Core.");

            // Release the managers, and release self
            self._release_managers ();
            App::_terminate ();
        }
    }

/*===============================================================================================*/
/*------APP PUBLIC STATIC METHODS----------------------------------------------------------------*/
/*===============================================================================================*/

    /// Returns a new App Builder.
    pub fn builder () -> AppBuilder {
        AppBuilder::new ()
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Checks if the app has been initialized.
    ///
    /// # Return value
    /// A bool returning whether the app has been initialized.
    pub fn is_initialized () -> bool {

        unsafe {

            match APP_POINTER {

                Some (pointer) => !pointer.is_null (),
                None => false
            }
        }
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a reference to the app instance.
    pub fn get_instance () -> Result<&'static App, ()> {

        if App::is_initialized () {
            return Ok (unsafe {&*APP_POINTER.unwrap ()});
        }

        Err (())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a mutable reference to the app instance.
    pub fn get_instance_mut () -> Result<&'static mut App, ()> {

        if App::is_initialized () {
            return Ok (unsafe {&mut *APP_POINTER.unwrap ()});
        }

        Err (())
    }

/*===============================================================================================*/
/*------APP PRIVATE METHODS----------------------------------------------------------------------*/
/*===============================================================================================*/

    // Initializes the various managers.
    fn _init_managers (&self) {

        self.resource_mgr.borrow_mut ().init ();
        self.window_mgr.borrow_mut   ().init ();
    }

/*-----------------------------------------------------------------------------------------------*/

    // Checks diretories for errors
    fn _check_dirs_for_errors (&self) {

        // Resource directory
        if !Path::new (&self.res_dir).exists () {
            panic! ("Resource path \"{}\" does not exist.\nApplication cannot continue.", &self.res_dir);
        }

        // Bin directory
        if !Path::new (&self.bin_dir).exists () {
            panic! ("Binary path \"{}\" does not exist.\nApplication cannot continue.", &self.bin_dir);
        }

        // Plugin directory
        if !Path::new (&self.plg_dir).exists () {
            panic! ("Plugin path \"{}\" does not exist.\nApplication cannot continue.", &self.plg_dir);
        }

        // Config directory
        if !Path::new (&self.cfg_dir).exists () {

            if let Err (e) = fs::create_dir_all (&self.cfg_dir) {
                panic! ("Config path \"{}\" could not be created.\n{}.\nApplication cannot continue.", &self.cfg_dir, e);
            }
        }
    }

/*-----------------------------------------------------------------------------------------------*/

    // TODO: Finish me
    // On pre render
    fn _on_pre_render (&self) {
        self.window_mgr.borrow_mut ().on_pre_render ();
    }

/*-----------------------------------------------------------------------------------------------*/

    // TODO: Finish me
    // On render
    fn _on_render (&self) {
        self.window_mgr.borrow_mut ().on_render ();
    }

/*-----------------------------------------------------------------------------------------------*/

    // TODO: Finish me
    // On post render
    fn _on_post_render (&self) {
        self.window_mgr.borrow_mut ().on_post_render ();
    }

/*-----------------------------------------------------------------------------------------------*/

    // Releases the managers
    fn _release_managers (&self) {
        self.window_mgr.borrow_mut ().release ();
    }

/*===============================================================================================*/
/*------APP PRIVATE STATIC METHODS---------------------------------------------------------------*/
/*===============================================================================================*/

    // Release the app pointer, and terminate the application.
    fn _terminate () {

        unsafe {

            drop (Box::from_raw (APP_POINTER.unwrap ()));
            APP_POINTER = None;
        };

        // Release the logger, and shutdown the application
        info! ("Terminating the application.");
        Logger::release ();

        process::exit (0);
    }
}

/*===============================================================================================*/
/*------APP BUILDER STRUCT-----------------------------------------------------------------------*/
/*===============================================================================================*/

/// Used for building a new, static app instance.
pub struct AppBuilder {

    // Private
    _project_name: String,
    _project_developer: String,
    _project_version: Version,
}

/*===============================================================================================*/
/*------APP BUILDER PUBLIC METHODS---------------------------------------------------------------*/
/*===============================================================================================*/

impl AppBuilder {

    /// Sets the app name.
    pub fn project_name (&mut self, name: &str) -> &mut Self {

        self._project_name = name.to_string ();
        self
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Sets the app developer.
    pub fn project_developer (&mut self, developer: &str) -> &mut Self {

        self._project_developer = developer.to_string ();
        self
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Sets the app version
    pub fn project_version (&mut self, version: Version) -> &mut Self {

        self._project_version = version;
        self
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Builds the app.
    pub fn build (&self) -> Result<&'static mut App, ()> {

        // Check if initialized
        if !App::is_initialized () {

            let ab = Box::new (App {

                resource_mgr:      Rc::new (RefCell::new (ResourceManager::new ())),
                window_mgr:        Rc::new (RefCell::new (WindowManager::new ())),

                project_name:      self._project_name.clone (),
                project_developer: self._project_developer.clone (),
                project_version:   self._project_version,

                res_dir: "res/".to_string (),
                cfg_dir: format! ("{}/.{}/{}/", env::home_dir ().unwrap ().display (), &self._project_developer, &self._project_name),
                bin_dir: "bin/".to_string (),
                plg_dir: "bin/plugins/".to_string (),

                _is_in_main_loop:  false,
                _should_exit:      false,
            });

            unsafe {APP_POINTER = Some (Box::into_raw (ab))};
        }

        App::get_instance_mut ()
    }

/*===============================================================================================*/
/*------APP BUILDER PUBLIC STATIC METHODS--------------------------------------------------------*/
/*===============================================================================================*/

    /// Returns a new app builder instance.
    pub fn new () -> AppBuilder {

        AppBuilder {

            _project_name:      "Untitled".to_string (),
            _project_developer: "Unknown".to_string (),
            _project_version:   Version::default (),
        }
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Default for AppBuilder {

    fn default () -> AppBuilder {
        AppBuilder::new ()
    }
}
