using CsaCommon;
using Google.Protobuf;
using Google.Protobuf.WellKnownTypes;
using Grpc.Core;
using Microsoft.AspNetCore.Builder;
using Microsoft.Extensions.DependencyInjection;
using TestApi;

namespace TestApp;

public class Program
{
    public static void Main(string[] args)
    {
        var builder = WebApplication.CreateBuilder(args);
        builder.Services.AddGrpc(static o => { o.EnableDetailedErrors = true; }).AddJsonTranscoding();
        using var app = builder.Build();
        app.UseGrpcWeb();
        app.MapGrpcService<TestService>().EnableGrpcWeb();
        app.Run();
    }
}

public class TestService : TestApi.TestService.TestServiceBase
{
    public override Task<EmptyTest> Empty(EmptyTest request, ServerCallContext context)
    {
        throw new RpcException(new Status(StatusCode.Unimplemented, ""));
    }

    public override Task<EmptyTest> Primative(PrimativeTest request, ServerCallContext context)
    {
        var status = new Google.Rpc.Status
        {
            Code = (int)StatusCode.NotFound,
            Message = "Simple error message",
            Details = { Any.Pack(new BadRequestDetail { Property = "EthanRamsey", Message = "LessThanOrEqual" }) }
        };

        throw status.ToRpcException();

        // var trailers = new Metadata();
        // trailers.Add("grpc-status-details-bin", status.ToByteArray());
        // context.WriteResponseHeadersAsync(headers);
        // context.ResponseTrailers.Add("grpc-status-details-bin", status.ToByteArray());
        // throw new RpcException(new Status(StatusCode.NotFound, ""), trailers);

        // try
        // {
        //     request.ToModel();
        //     return Task.FromResult(new EmptyTest());
        // }
        // catch (InvalidArgumentsException ex)
        // {
        //     var status = new Google.Rpc.Status
        //     {
        //         Code = (int)StatusCode.NotFound,
        //         Message = "Simple error message",
        //         Details = { Any.Pack(new Google.Rpc.ErrorInfo { Domain = "example", Reason = "some reason" }) }
        //     };

        //     throw status.ToRpcException();
        // }
    }

    public override Task<EmptyTest> Nullable(NullableTest request, ServerCallContext context)
    {
        throw new RpcException(new Status(StatusCode.Unimplemented, ""));
    }

    public override Task<EmptyTest> Repeated(RepeatedTest request, ServerCallContext context)
    {
        throw new RpcException(new Status(StatusCode.Unimplemented, ""));
    }

    public override Task<EmptyTest> RepeatedNullable(RepeatedNullableTest request, ServerCallContext context)
    {
        throw new RpcException(new Status(StatusCode.Unimplemented, ""));
    }
}