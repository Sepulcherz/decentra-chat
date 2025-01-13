use wasm_bindgen::prelude::*;
use web_sys::{console, window};
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ethereum)]
    fn request(args: JsValue) -> js_sys::Promise;
}

#[derive(Serialize, Deserialize, Clone)] // Added Clone here
struct Profile {
    address: String,
    username: String,
    bio: String,
    posts: Vec<Post>,
    following: Vec<String>,
    followers: Vec<String>,
    github_username: Option<String>,
    github_repos: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone)] // Added Clone here
struct Post {
    id: String,
    content: String,
    author: String,
    timestamp: u64,
    likes: Vec<String>,
}

#[wasm_bindgen]
pub struct DecentraChat {
    current_account: Option<String>,
    profiles: Vec<Profile>,
    posts: Vec<Post>,
}

#[wasm_bindgen]
impl DecentraChat {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console::log_1(&"Initializing DecentraChat...".into());
        
        Self {
            current_account: None,
            profiles: Vec::new(),
            posts: Vec::new(),
        }
    }
    pub async fn connect_github(&mut self, github_username: String) -> Result<(), JsValue> {
    // TODO: Implement GitHub connection
    // Requirements:
    // 1. Validate GitHub username
    // 2. Fetch public repositories
    // 3. Store GitHub data in profile
    // 4. Add error handling
    // Bounty Value: 0.1 ETH
    Err(JsValue::from_str("Not implemented"))
    }

    pub async fn connect_wallet(&mut self) -> Result<JsValue, JsValue> {
        let window = window().unwrap();
        
        if js_sys::Reflect::has(&window, &"ethereum".into())? {
            let params = js_sys::Array::new();
            params.push(&"eth_requestAccounts".into());
            
            let request_obj = js_sys::Object::new();
            js_sys::Reflect::set(&request_obj, &"method".into(), &"eth_requestAccounts".into())?;
            js_sys::Reflect::set(&request_obj, &"params".into(), &params)?;

            let accounts = wasm_bindgen_futures::JsFuture::from(request(JsValue::from(request_obj))).await?;
            let accounts: js_sys::Array = accounts.into();
            
            if accounts.length() > 0 {
                let account = accounts.get(0).as_string().unwrap();
                self.current_account = Some(account.clone());
                
                if !self.profiles.iter().any(|p| p.address == account) {
                    self.create_profile(&account, "Anonymous", "No bio yet");
                }
                
                Ok(JsValue::from_str(&account))
            } else {
                Err(JsValue::from_str("No accounts found"))
            }
        } else {
            Err(JsValue::from_str("Please install MetaMask"))
        }
    }

    pub fn create_profile(&mut self, address: &str, username: &str, bio: &str) {
        let profile = Profile {
            address: address.to_string(),
            username: username.to_string(),
            bio: bio.to_string(),
            posts: Vec::new(),
            following: Vec::new(),
            followers: Vec::new(),
        };
        
        self.profiles.push(profile);
    }

    pub fn create_post(&mut self, content: &str) -> Result<(), JsValue> {
        if let Some(account) = &self.current_account {
            let post = Post {
                id: format!("post_{}", self.posts.len()),
                content: content.to_string(),
                author: account.clone(),
                timestamp: js_sys::Date::now() as u64,
                likes: Vec::new(),
            };
            
            // Add to global posts
            self.posts.push(post.clone());
            
            // Add to author's posts
            if let Some(profile) = self.profiles.iter_mut().find(|p| &p.address == account) {
                profile.posts.push(post);
            }
            
            Ok(())
        } else {
            Err(JsValue::from_str("Not connected"))
        }
    }

    pub fn like_post(&mut self, post_id: &str) -> Result<(), JsValue> {
        if let Some(account) = &self.current_account {
            if let Some(post) = self.posts.iter_mut().find(|p| p.id == post_id) {
                if !post.likes.contains(account) {
                    post.likes.push(account.clone());
                }
            }
            Ok(())
        } else {
            Err(JsValue::from_str("Not connected"))
        }
    }

    pub fn follow_user(&mut self, target_address: &str) -> Result<(), JsValue> {
        if let Some(account) = &self.current_account {
            if account == target_address {
                return Err(JsValue::from_str("Cannot follow yourself"));
            }

            if let Some(profile) = self.profiles.iter_mut().find(|p| &p.address == account) {
                if !profile.following.contains(&target_address.to_string()) {
                    profile.following.push(target_address.to_string());
                }
            }

            if let Some(profile) = self.profiles.iter_mut().find(|p| p.address == target_address) {
                if !profile.followers.contains(account) {
                    profile.followers.push(account.clone());
                }
            }

            Ok(())
        } else {
            Err(JsValue::from_str("Not connected"))
        }
    }

    pub fn get_profile(&self, address: &str) -> JsValue {
        if let Some(profile) = self.profiles.iter().find(|p| p.address == address) {
            serde_json::to_string(profile).map(|s| JsValue::from_str(&s)).unwrap_or(JsValue::null())
        } else {
            JsValue::null()
        }
    }

    pub fn get_posts(&self) -> JsValue {
        serde_json::to_string(&self.posts).map(|s| JsValue::from_str(&s)).unwrap_or(JsValue::null())
    }

    pub fn get_current_account(&self) -> Option<String> {
        self.current_account.clone()
    }
}