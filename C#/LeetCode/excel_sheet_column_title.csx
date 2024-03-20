public class Solution
{
    public string ConvertToTitle(int columnNumber) {
        var res = "";
        while (columnNumber > 0) {
            columnNumber--;
            res = (char)('A' + (columnNumber % 26)) + res;
            columnNumber /= 26;
        }
        return res;
    }
}