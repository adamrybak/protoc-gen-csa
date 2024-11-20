using Google.Protobuf;

namespace CsaCommon;

public interface IToModel<T>
{
    T ToModel(string? propertyPath = null);
}

public interface IToMessage<T> where T : IMessage<T>
{
    T ToMessage();
}
