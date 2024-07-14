use crate::GTK::utilities::*;
use std::collections::HashMap;

#[derive(PartialEq)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct uGraph
{
	pub adjlists :Vec<Vec<i32>>,
	pub order:i32,
}
impl uGraph
{
	pub fn new(order : i32) -> Self
	{
		uGraph
		{
			adjlists : vec![vec![] ; order as usize],
			order,
		}
	}
	
	pub fn push(&mut self,start : i32 , end :i32)
	{
		if start>= self.order ||end >= self.order ||start < 0 ||end < 0 
		{
			message("not found", "not a vertex");
			return
		}
		if start == end 
		{
			message("same number","can't add edge to itself");
			return 
		}
		for i in 0..self.adjlists[start as usize].len()
		{
			if self.adjlists[start as usize][i] == end
			{
				message("error","already in the graph");
				return
			}
		}
		for i in 0..self.adjlists[end as usize].len()
		{
			if self.adjlists[end as usize][i] == start
			{
				message("error","already in the graph");
				return
			}
		}
		self.adjlists[start as usize].push(end);
		self.adjlists[start as usize].sort();
		self.adjlists[end as usize].push(start);
		self.adjlists[end as usize].sort();
	}
}		


#[derive(PartialEq)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct ucGraph
{
	pub adjlists :Vec<Vec<i32>>,
	pub order:i32,
	pub costs : HashMap<(i32,i32),i32>,
}
impl ucGraph
{
	pub fn new(order : i32) -> Self
	{
		ucGraph
		{
			adjlists : vec![vec![] ; order as usize],
			order,
			costs: HashMap::new(),
		}
	}
	 
	pub fn push(&mut self,start : i32 , end :i32 , cost : i32 )
	{
		if end >= self.order || start >= self.order || end < 0 || start <0
		{
			message("not found", "not a vertex");
			return
		}
		if start == end 
		{
			message("same number","can't add edge to itself");
			return 
		}
		for i in 0..self.adjlists[start as usize].len()
		{
			if self.adjlists[start as usize][i] == end
			{
				message("error","already in the graph");
				return
			}
		}
		for i in 0..self.adjlists[end as usize].len()
		{
			if self.adjlists[end as usize][i] == start
			{
				message("error","already in the graph");
				return
			}
		}
		self.adjlists[start as usize].push(end);
		self.adjlists[start as usize].sort();
		self.adjlists[end as usize].push(start);
		self.adjlists[end as usize].sort();
		if start < end
		{
			self.costs.insert((start,end),cost);
		}
		else
		{
			self.costs.insert((end,start),cost);
		}
	 }
}
#[derive(PartialEq)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct diGraph
{
	pub adjlists :Vec<Vec<i32>>,
	pub order:i32,
}
impl diGraph
{
	pub fn new(order : i32) -> Self
	{
		diGraph
		{
			adjlists : vec![vec![] ; order as usize],
			order,
		}
	}
	
	pub fn push(&mut self,start : i32 , end :i32)
	{
		if end >= self.order || start >= self.order || end < 0 || start <0
		{
			message("not found", "not a vertex");
			return
		}
		if start == end 
		{
			message("same number","can't add edge to itself");
			return 
		}
		for i in 0..self.adjlists[start as usize].len()
		{
			if self.adjlists[start as usize][i] == end
			{
				message("error","already in the graph");
				return
			}
		}
		self.adjlists[start as usize].push(end);
		self.adjlists[start as usize].sort();
	}
}		


#[derive(PartialEq)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct dicGraph
{
	pub adjlists :Vec<Vec<i32>>,
	pub order:i32,
	pub costs : HashMap<(i32,i32),i32>,
}
impl dicGraph
{
	pub fn new(order : i32) -> Self
	{
		dicGraph
		{
			adjlists : vec![vec![] ; order as usize],
			order,
			costs: HashMap::new(),
		}
	}
	
	pub fn push(&mut self, start : i32 , end :i32 , cost : i32 )
	{
		if end >= self.order || start >= self.order || end < 0 || start <0
		{
			message("not found", "not a vertex");
			return
		}
		if start == end 
		{
			message("same number","can't add edge to itself");
			return 
		}        
		for i in 0..self.adjlists[start as usize].len()
		{
			if self.adjlists[start as usize][i] == end
			{
				message("error","already in the graph");
				return
			}
		}
		self.adjlists[start as usize].push(end);
		self.adjlists[start as usize].sort();
		self.costs.insert((start,end),cost);
	}
}
