//
//  PrefixerExceptions.h
//  Pods
//
//  Created by geal on 31/07/16.
//
//

#import <Foundation/Foundation.h>
static NSString *InvalidArgumentPrefixerException = @"InvalidArgumentPrefixerException";
static NSString *EmptyInputPrefixerException      = @"EmptyInputPrefixerException";
static NSString *NotEnoughRoomPrefixerException   = @"NotEnoughRoomPrefixerException";
static NSString *WriteErrorPrefixerException      = @"WriteErrorPrefixerException";

@interface PrefixerExceptions : NSObject
+(void)interpretErrorCode:(uint)code;
@end
