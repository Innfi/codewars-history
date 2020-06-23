/*
https://www.codewars.com/kata/546f922b54af40e1e90001da

one of the optimal solutions was:

using System.Linq;
public static class Kata
{
  public static string AlphabetPosition(string text)
  {
     return string.Join(" ", text.ToLower().Where(char.IsLetter).Select(x => x - 'a'+1));
  }
}

*/

using System.Linq;

public static class Kata
{
    public static string AlphabetPosition(string text)
    {
        return string.Join(" ", 
            text.toLower()
            .Where(x => char.IsLetter(x))
            .Select(x => (x - 96))
            .ToArray());
    }
}

