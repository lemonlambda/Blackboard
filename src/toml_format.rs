use serde_derive::Deserialize;


#[derive(Deserialize)]
pub struct Config 
{
	pub
	  package : Package,
	pub
	  tools : Tools,
	pub
	  meta : Meta,
	pub
	  compile : Compile,
	pub
	  linking : Linking,
}

#[derive(Deserialize)]
pub struct Package 
{
	pub
	  name : String,
	pub
	  version : String,
}

#[derive(Deserialize)]
pub struct Tools 
{
	pub
	  compiler : Option<String>,
	pub
	  linker : Option<String>,
}

#[derive(Deserialize)]
pub struct Meta 
{
	pub
	  src_files : Option<String>,
	pub
	  header_files : Option<String>,
}

#[derive(Deserialize)]
pub struct Compile 
{
	pub
	  before : Option<String>,
	pub
	  run : Option<String>,
	pub
	  after : Option<String>,
}

#[derive(Deserialize)]
pub struct Linking 
{
	pub
	  before : Option<String>,
	pub
	  run : Option<String>,
	pub
	  after : Option<String>,
}
