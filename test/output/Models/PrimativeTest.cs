using CsaCommon;

namespace TestApi.Models;

public partial class PrimativeTest
{
    public void Validate()
    {
        if (EthanRamsey <= 0)
        {
            throw InvalidArgumentsException.LessThanOrEqual(null, nameof(EthanRamsey), 0);
        }
    }
}
