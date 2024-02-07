public class Solution
{
    public static int LengthOfLongestSubstring(string s) {
        if (s.Length == 0) return 0;
        if (s.Length == 1) return 1;
        var max = 1;
        var start = 0;
        var end = 1;
        var dict = new Dictionary<char, int> { [s[0]] = 0 };
        while (end < s.Length) {
            if (dict.ContainsKey(s[end]) && dict[s[end]] >= start) {
                start = dict[s[end]] + 1;
            }
            dict[s[end]] = end;
            max = Math.Max(max, end - start + 1);
            end++;
        }
        return max;
    }
}

var res1 = Solution.LengthOfLongestSubstring("abcabcbb");
var res2 = Solution.LengthOfLongestSubstring("bbbbb");
var res3 = Solution.LengthOfLongestSubstring("pwwkew");

Console.WriteLine(res1);
Console.WriteLine(res2);
Console.WriteLine(res3);