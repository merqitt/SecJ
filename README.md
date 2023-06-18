# SecJ

Insufficient Input Validation (IIV)
Query String Vulnerability - Rust

Input validation is a frequently-used technique for checking potentially dangerous inputs in order to ensure that the inputs are safe for processing within the code, or when communicating with other components. When software does not validate input properly, an attacker is able to craft the input in a form that is not expected by the rest of the application. This will lead to parts of the system receiving unintended input, which may result in unintended output. (CWE)


### Clone Repository 

sudo apt install git

git clone https://github.com/merqitt/SecJ.git 

cd SecJ

### Install

chmod +x install.sh

./install.sh

source "$HOME/.cargo/env"

### Run

cd vuln_App

cargo run 

open http://127.0.0.1:8000

http://127.0.0.1:8000/passwords.txt

![Screenshot from 2023-06-18 14-58-43](https://github.com/merqitt/SecJ/assets/90560259/c4de92ff-e81b-4dcd-b7bf-6c47b9283cb0)


### Run

cd App  

cargo run 

open http://127.0.0.1:8000

http://127.0.0.1:8000/passwords.txt

![Screenshot from 2023-06-18 14-45-39](https://github.com/merqitt/SecJ/assets/90560259/1562d9e4-396d-47db-8939-a3d4060f4a62)




