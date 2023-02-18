using System.Collections;
using System.Collections.Generic;
using System.Linq;

namespace Solution;

public class ListFilterer
{
    public static IEnumerable<int> GetIntegersFromList(List<object> listOfItems)
    {
        // Lambda goes brrr
        return listOfItems.Where(value => value is int).Select(value => (int)value).ToList();

    }
}
