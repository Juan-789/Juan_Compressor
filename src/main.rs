use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::collections::BinaryHeap;
use std:: fs::{read_to_string, write};

#[derive(Debug, PartialEq, Eq)]
struct HuffmanNode{
    count: u32,
    c: Option<char>,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}
struct BitWriter{
    buffer: u8,
    bit_count: u8,
    bytes: Vec<u8>,
}

impl BitWriter{
    fn new() -> Self{
        BitWriter {buffer: 0, bit_count: 0, bytes: Vec::new()}
    }
    fn write_bit(&mut self, bit: bool) {
        self.buffer = self.buffer << 1;
        if bit {
            self.buffer |= 1;
        }
        self.bit_count += 1;
    
        if self.bit_count == 8 {
            self.bytes.push(self.buffer);
            self.buffer = 0;
            self.bit_count = 0;
        }
    }
    fn flush(&mut self) -> Vec<u8> {
        if self.bit_count > 0 {
            self.buffer = self.buffer << (8 - self.bit_count);
            self.bytes.push(self.buffer);
        }
        self.bytes.clone()
    }
}





fn main()-> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();
    let argc = args.len();
    if argc == 1 {
        println!("not enough arguments passed");
        return Err("Not given a file".into());
    }
    let file_path = &args[1];
    
    let contents = read_to_string(file_path)?;

    println!("file contents");
    println!("{}", contents);
    let huffman_tree: Option<HuffmanNode> = huffman_tree_builder(contents.clone());

    if let Some(root_node) = huffman_tree {
        // 1. Create the empty map to hold our dictionary
        let mut code_table: HashMap<char, String> = HashMap::new();

        // 2. Kick off the recursion
        // We start with an empty string ""
        generate_binary_table(&root_node, String::new(), &mut code_table);

        // 3. Print the results
        println!("--- Huffman Code Table ---");
        for (char_key, binary_code) in &code_table {
            println!("'{}' : {}", char_key, binary_code);
        }
        let mut compressed = String::new();
        for meow in contents.chars() {
            //maybe one character doesnt get translated
            compressed.push_str(&code_table.get(&meow).unwrap());
        }
        let mut writer = BitWriter::new();
        
        for c in compressed.chars(){
            match c{
                '1' => writer.write_bit(true),
                '0' => writer.write_bit(false),
                _ => panic!("not a bit"),
            }
        }
        writer.flush();
        //now lets make the binary representation of it
        write("out_success.juan", &writer.bytes);
    } else {
        println!("The tree was empty!");
    }
    return Ok(());
}



fn huffman_tree_builder(contents: String) -> Option<HuffmanNode> { //makes the string into a map of numbers
    let mut huffman_tree: HashMap<char, u32> = HashMap::new();
    for c in contents.chars() { //this  also takes the \n (newline)
        let count = huffman_tree.entry(c).or_insert(0);
        *count += 1;
    }

    let mut min_heap = BinaryHeap::new();
    //now that we have the map, lets build the min heap
    for k in huffman_tree.keys() {
        println!("key:{} count:{:?}", k, huffman_tree.get(k));
        min_heap.push(HuffmanNode{count: *huffman_tree.get(k).unwrap(), c: Some(*k), left: None, right: None})
    }
    println!("{:?}", min_heap);
    while min_heap.len()>1 {
        let left = min_heap.pop().unwrap();
        let right = min_heap.pop().unwrap();

        let merged = Box::new(HuffmanNode{
            count: left.count+right.count,
            c: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        });
        min_heap.push(*merged);
    }
    let root = min_heap.pop();
    // now we make our tree
    println!("tree ts");

    println!("{:#?}", root);
    return root;
}

fn generate_binary_table(node: &HuffmanNode, prefix: String, map: &mut HashMap<char, String>) {
    if let Some(ch) = node.c {
        map.insert(ch, prefix);
    } else {
        if let Some(ref left_child) = node.left {
            generate_binary_table(left_child, format!("{}0", prefix), map);
        }
        if let Some(ref right_child) = node.right {
            generate_binary_table(right_child, format!("{}1", prefix), map);
        }
    }
}


impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.count.cmp(&self.count)
            .then_with(|| self.c.cmp(&other.c))
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}