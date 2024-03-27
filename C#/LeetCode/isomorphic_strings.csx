public class Solution
{
    public bool IsIsomorphic(string s, string t) {
        Dictionary<char, char> StoT = [];
        Dictionary<char, char> TtoS = [];
        for (int i = 0; i < s.Length; i++) {
            if (StoT.ContainsKey(s[i]) && (StoT[s[i]] != t[i]) ||
                TtoS.ContainsKey(t[i]) && (TtoS[t[i]] != s[i])) {
                return false;
            }
            StoT[s[i]] = t[i];
            TtoS[t[i]] = s[i];
        }
        return true;
    }
}