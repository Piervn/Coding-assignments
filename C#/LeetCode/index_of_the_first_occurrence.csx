// Implementation of the KMP Algorithm
// Time complexity: O(n)
public class Solution
{
    public static int[] computePrefixFunction(string pattern) {
        int m = pattern.Length;
        int[] pi = new int[m];
        pi[0] = -1;
        int k = -1;
        for (int q = 1; q < m; q++) {
            while (k >= 0 && pattern[k + 1] != pattern[q]) k = pi[k];
            if (pattern[k + 1] == pattern[q]) k++;
            pi[q] = k;
        }
        return pi;
    }

    public static int StrStr(string haystack, string needle) {
        int m = needle.Length;
        int[] pi = computePrefixFunction(needle);
        int q = -1;
        for (int i = 0; i < haystack.Length; i++) {
            while (q >= 0 && needle[q + 1] != haystack[i]) q = pi[q];
            if (needle[q + 1] == haystack[i]) q++;
            if (q == m - 1) return i - m + 1;
        }
        return -1;
    }
}

var test1 = ("sadbutsad", "sad");
var test2 = ("leetcode", "leeto");

Console.WriteLine(Solution.StrStr(test1.Item1, test1.Item2));
Console.WriteLine(Solution.StrStr(test2.Item1, test2.Item2));
