/*

https://www.codewars.com/kata/5262119038c0985a5b00029f

*/

using System;
using System.Linq;

public static class Kata
{
  public static bool IsPrime(int n)
  {
      if(n <= 1) return false;
      if(n == 2) return true;
      
      var squareRoot = (int)Math.Sqrt(n);
      var candidates = Enumerable.Range(2, squareRoot);

      foreach (var divisor in candidates)
      {
          if (n % divisor == 0) return false;
      }

      return true;
  }
}