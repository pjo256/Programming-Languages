
class OInt(x: Int) extends Ordered[OInt] {
	val n = x;
	def compare(that: OInt): Int = this.n compare that.n;
	override def toString(): String = "<" + n + ">";
}

abstract class OTree[T <: Ordered[T]] extends Ordered[OTree[T]];

case class ONode[T <: Ordered[T]](children: List[OTree[T]]) extends OTree[T] {
	def compare(that: OTree[T]): Int = {
		that match {
			case ONode(list) => Comparer.lexicographic(this.children, list);
			case OLeaf(value) => 1;
		}
	}

	override def toString(): String = "ONode(" + children.toString + ")";
}

case class OLeaf[T <: Ordered[T]](value: T) extends OTree[T] {
	def compare(that: OTree[T]): Int = {
		that match {
			case ONode(list) => -1;
			case OLeaf(value) => this.value compare value;
		}
	}
	override def toString(): String = "OLeaf(" + value.toString + ")";
}

object Comparer {

	def lexicographic[T <: Ordered[T]](n : List[T], m : List[T]): Int = {
		(n, m) match {
			case (Nil, Nil) => 0;
			case (x::xs, Nil) => 1;
			case (Nil, y::ys) => -1;
			case (x::xs, y::ys) => (x compare y) match {
				case 0 => lexicographic(xs, ys);
				case v => v;
			}
		}
	}
}

object ODemo {

	def compareTrees[T <: Ordered[T]](n: OTree[T], m: OTree[T]) {
		(n compare m) match {
			case -1 => println("Less")
			case 0 => println("Equal")
			case 1 => println("Greater")
			case 3 => println("Still buggy")
		}
	}

	def test() {

    	val tree1 = ONode(List(OLeaf(new OInt(6))))

    	val tree2 = ONode(List(OLeaf(new OInt(3)),
			   OLeaf(new OInt(4)), 
			   ONode(List(OLeaf(new OInt(5)))), 
			   ONode(List(OLeaf(new OInt(6)), 
				      OLeaf(new OInt(7))))));

    	val treeTree1: OTree[OTree[OInt]] = 
      		ONode(List(OLeaf(OLeaf(new OInt(1)))))

    	val treeTree2: OTree[OTree[OInt]] = 
      		ONode(List(OLeaf(OLeaf(new OInt(1))),
		 		OLeaf(ONode(List(OLeaf(new OInt(2)), 
				  OLeaf(new OInt(2)))))))


   		print("tree1: ")
    	println(tree1)
    	print("tree2: ")
    	println(tree2)
    	print("treeTree1: ")
    	println(treeTree1)
    	print("treeTree2: ")
    	println(treeTree2)
    	print("Comparing tree1 and tree2: ")
    	compareTrees(tree1, tree2)
    	print("Comparing tree2 and tree2: ")
    	compareTrees(tree2, tree2)
    	print("Comparing tree2 and tree1: ")
    	compareTrees(tree2, tree1)
    	print("Comparing treeTree1 and treeTree2: ")
    	compareTrees(treeTree1, treeTree2)
    	print("Comparing treeTree2 and treeTree2: ")
    	compareTrees(treeTree2, treeTree2)
    	print("Comparing treeTree2 and treeTree1: ")
    	compareTrees(treeTree2, treeTree1)
    }

	def main(args: Array[String]) {
		test();
	}
}