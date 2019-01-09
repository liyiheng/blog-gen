class Solution:
    def generate(self, numRows):
        """
        :type numRows: int
        :rtype: List[List[int]]
        """
        result = []
        if numRows == 0:
            return result
        result.append([1])
        if numRows == 1:
            return result
        for i in range(1, numRows):
            prev = result[i-1]
            cur = []
            for j in range(0, i+1):
                a = 0
                b = 0
                if j-1 >= 0:
                    a = prev[j-1]
                if j < len(prev):
                    b = prev[j]
                cur.append(a+b)
            result.append(cur)
        return result

s = Solution()
result = s.generate(5)
print(result)
