# Deployment Plan

## Task Matrix
| Task / Section            | SualpGdk |
|--------------------------|----------|
| Introduction             |     x     | 
| Deployment Overview      |     x     |  
| Deployment Process       |     x     |  
| Configuration Plan       |     x     |    
| Documentation Formatting |     x     | 

## List Of Contributors
| Name      | Role                  |
|-----------|-----------------------|
| SualpGdk  |     Author            |


## Table Of Contents
| Section Number | Section Name          |
|----------------|----------------------|
| 2              | Deployment Overview  |
| 3              | Deployment Process   |
| 4              | Configuration Plan   |

## Deployment Overview
The project was deployed locally for the demo using Dioxus framework. The application was built using Rust language and compiled using Cargo. deployment
enviroments are flexible but since the primary project of the project is local, the application is runned locally on an OS.

## Deployment Process
- Ensure that Dioxus CLI and Rust toolchain is intstalled.
- Ensure all project files and source code is installed.
- On terminal(or your code editor), navigate to main project folder in the root directory of the project.
- Run command "dx serve".
- Wait for the build process to complete.

## Configuration Plan
The project uses the Dioxus framework with its default configuration for local deployment as a native desktop application. The application is executed
using the dx serve command, which builds and runs the project directly on the operating system. No additional environment variables or custom configuration
files were required. Dependency management is handled by Cargo, and the project runs using default development settings for demonstration purposes.
