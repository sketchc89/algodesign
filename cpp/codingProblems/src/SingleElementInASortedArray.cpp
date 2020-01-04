/*! Search for an element [lo,hi) that is less than preceding element and greater than following
 * element
 *  @param vi sorted array of numbers where one element is unique
 *  @param lo index to search above inclusive
 *  @param hi index to search below exclusive
 *  @return unique element if found, -1 otherwise
 */

#include <vector>

#include "Problems.h"

using namespace std;

int uniqueBinarySearch(const vector<int>& vi, int lo, int hi) {
    if (lo >= hi) {
        return -1;  // FAIL
    }
    int mid = (lo + hi) / 2;
    if ((mid == 0 || vi[mid] > vi[mid - 1])
        && (static_cast<size_t>(mid) == vi.size() - 1 || vi[mid] < vi[mid + 1])) {
        return mid;  // SUCCESS
    }
    int loSearch = uniqueBinarySearch(vi, 0, mid);
    int hiSearch = uniqueBinarySearch(vi, mid + 1, hi);
    if (loSearch > 0) {
        return loSearch;  // SUCCESS at lower level
    } else if (hiSearch > 0) {
        return hiSearch;  // SUCCESS at lower level
    } else {
        return -1;  // FAIL
    }
}

int uniqueBinarySearch(const vector<int>& vi) {
    return uniqueBinarySearch(vi, 0, vi.size());
}
