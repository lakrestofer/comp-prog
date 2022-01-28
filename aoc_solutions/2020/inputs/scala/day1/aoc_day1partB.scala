object Main {

  def main(args: Array[String]): Unit = if (args.size == 1) {
    val inputfilepath = args(0)
    val lines: Vector[Int] = scala.io.Source.fromFile(inputfilepath)
                                .getLines
                                .toVector
                                .map(str => str.toInt)
                                .sorted

    def findTwoThatSumToN(xs: Vector[Int], n: Int): (Int, Int) = {
      var haveFoundPair: Boolean = false
      var i: Int = 0
      var validPair: (Int, Int) = (0,0)
      while (!haveFoundPair && (i < xs.length)) {
        println(s"Testing for any pairs of ${xs(i)}")
        var j: Int = i + 1
        while (!haveFoundPair && (j < xs.length)) {
          if (xs(i) + xs(j) == n){
            haveFoundPair = true
            validPair = (xs(i), xs(j))
            println(s"found a valid pair. It is ${validPair}")
          }
          j += 1
        }
        i +=1
      }
      validPair
    }

    def findThreeThatSumToN(xs: Vector[Int], n: Int): (Int, Int, Int) = {
      var haveFoundTriplet: Boolean = false
      var i: Int = 0
      var validTriplet: (Int, Int, Int) = (0,0,0)
      while (!haveFoundTriplet && (i < xs.length)) {
        println("looking for pair to add to xs(i)")
        val nLeft = n - xs(i)
        val potentialPair = findTwoThatSumToN(xs, nLeft)
        if (xs(i) + potentialPair._1 + potentialPair._2 == n) {
          haveFoundTriplet = true
          validTriplet = (xs(i), potentialPair._1, potentialPair._2)
        }
        i += 1
      }
      validTriplet
    }

    val tripletThatSumTo2020: (Int, Int, Int) = findThreeThatSumToN(lines, 2020)

    println(s"The product of the triplet the sums to 2020 is ${tripletThatSumTo2020._1 * tripletThatSumTo2020._2 * tripletThatSumTo2020._3}")

  } else {
    println("Must have path to inputfile as argument only")
  }
}
