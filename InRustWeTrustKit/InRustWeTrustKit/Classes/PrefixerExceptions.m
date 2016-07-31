//
//  PrefixerExceptions.m
//  Pods
//
//  Created by geal on 31/07/16.
//
//

#import "PrefixerExceptions.h"

@implementation PrefixerExceptions
+(void)interpretErrorCode:(uint)code {
    NSString*exceptionName;
    switch(code) {
        case 0:
            exceptionName = InvalidArgumentPrefixerException;
            break;
        case 1:
            exceptionName = EmptyInputPrefixerException;
            break;
        case 2:
            exceptionName = NotEnoughRoomPrefixerException;
            break;
        case 3:
            exceptionName = WriteErrorPrefixerException;
            break;
        default:
            @throw [NSException exceptionWithName:@"UnknownException" reason:@"invalid exception code" userInfo:nil];
    };
    
    @throw [NSException exceptionWithName:exceptionName reason:@"" userInfo:nil];
}
@end
