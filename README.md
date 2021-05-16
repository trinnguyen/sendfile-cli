# medusa-li

## Overview
- Send files securely via TCP/TLS 1.3 in local network

## Process
- Each receiver runs a TCP server to listen for connections from sender
- Initiate state machines for server and client, communicate using a custom protocol
- Custom protocol for TCP packets:

```
    <package> := <package_type> <data>?
    <package_type> := Send | Accept | Reject | StartFile | EndFile | FileData | Finish
    <data> := FileInfo[] | Byte[]
```


## State machines

### Receiver (or server)

```mermaid
stateDiagram-v2
    [*] --> Init
    Init --> InternalAnswer: Send?
    InternalAnswer --> Finish: Reject!
    InternalAnswer --> Accept: Accept!
    Accept --> StartReceivingFile: StartFile?
    StartReceivingFile --> ReceiveFileData: FileData?
    ReceiveFileData --> ReceiveFileData: FileData?
    ReceiveFileData --> EndReceivingFile: EndFile?
    EndReceivingFile --> StartReceivingFile: StartFile?
    EndReceivingFile --> Finish: Finish?
    Finish --> Init: tau
```

### Sender (or client)
```mermaid
stateDiagram-v2
    [*] --> Init
    Init --> WaitForResponse: Send!
    WaitForResponse --> Finish: Reject?
    WaitForResponse --> Accepted: Accept?
    Accepted --> StartSendingFile: StartFile!
    StartSendingFile --> SendFileData: FileData!
    SendFileData --> SendFileData: FileData!
    SendFileData --> EndSendingFile: EndFile!
    EndSendingFile --> StartSendingFile: StartFile!
    EndSendingFile --> Finish: Finish!
    Finish --> Init: tau
```