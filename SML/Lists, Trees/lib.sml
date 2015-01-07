Control.Print.printDepth := 100;
Control.Print.printLength := 100;


fun partition (op <) p [] tup = tup
|	partition (op <) p (x::xs) tup = 
		if p < x then partition (op <) p xs (#1tup, (x::(#2tup)))
		else partition (op <) p xs ((x::(#1tup)), #2tup);

fun intPartitionSort [] = []
|	intPartitionSort (x::xs) = 
		let 
			val (left, right) = partition (op <) x xs ([], [])
		in 
			intPartitionSort left @ [x] @ intPartitionSort right
		end;

fun partitionSort (op <) [] = []
|	partitionSort (op <) (x::xs) = 
		let 
			val (left, right) = partition (op <) x xs ([], [])
		in 
			partitionSort (op <) left @ [x] @ partitionSort (op <) right 
		end;

datatype 'a tree = leaf of 'a | node of 'a tree list;

fun sortTree (op <) (leaf xs) = leaf (partitionSort (op <) xs)
|   sortTree (op <) (node n) = node (map (sortTree (op <)) n);

fun merge (op <) [] [] = []
|	merge (op <) (x::xs) [] = (x::xs)
|	merge (op <) [] (x::xs) = (x::xs)
|	merge (op <) (x::xs) (y::ys) = 
		if x < y then 
			x::merge (op <) xs (y::ys)
		else 
			y::merge (op <) (x::xs) ys;

fun mergeTree (op <) (leaf xs) = xs
|	mergeTree (op <) (node n) = foldr (fn (x, y) => merge (op <) y (mergeTree (op <) (sortTree (op <) x))) [] n;


