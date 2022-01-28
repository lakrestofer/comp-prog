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

    val pairThatSumTo2020 = findTwoThatSumToN(lines, 2020)

    println(s"The product of the pair the sums to 2020 is ${pairThatSumTo2020._1 * pairThatSumTo2020._2}")

  } else {
    println("Must have path to inputfile as argument only")
  }
}
