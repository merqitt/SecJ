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

__________________________________________________________________________________________________________________________________________________________________________________

# Assignment Details

The goal of the assignment is to:

- [x] Select a security subject appropriate for training - Insufficient Input Validation
    
- [x] Create the lesson description. It should contain:
    
    - [x] A description and the importance of the subject (using real-world examples, if possible.)
        
    - [x] A narrative of how the subject will be taught. This would be a video lesson script, voice-over for an animation or written transcript of a lesson.
        
- [x] For the code exercise, show the vulnerable code that would be presented to the learner.
    
    - [x] Provide the learner with a description how best to solve the vulnerability (can be multiple ways, can use code examples)
        
    - [x] Show the code after the vulnerability has been removed.
        

Additional Information:

- You may use whatever programming language you would like. - **Rust**
    
- You can use anything for the lesson description—a google doc, word, markdown, etc. - **Joplin**
    

## Suggested Subjects/Vulnerabilities

- **Insufficient Input Validation**

## Part Two: Vulnerable Application Example

The goal of the assignment is to:

- [x] create an application that contains the vulnerability
    
- [x] create the patch to the vulnerability to resolve it

__________________________________________________________________________________________________________________________________________________________________________________

# TODO
- [] Future Todos - Rust Juice Shop
- [] Implement POST functionality
- [] Login/Logout feature
- 
