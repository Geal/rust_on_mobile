//
//  NSDataPrefixer.m
//  Pods
//
//  Created by geal on 31/07/16.
//
//

#import "NSDataPrefixer.h"
#import "PrefixerExceptions.h"

@implementation NSDataPrefixer
- (instancetype)initWithData:(NSData*) data {
    self = [super init];
    
    _prefixer = prefixer_new([data bytes], [data length]);
    return self;
}

-(void) dealloc {
    prefixer_free(_prefixer);
}

-(NSData*)prefixData:(NSData*) data {
    size_t len   = prefixer_output_len(_prefixer, [data length]);
    void* buffer = malloc(len);
    
    uint32_t error = 0;
    if (!prefixer_prefix(_prefixer,
                    [data bytes], [data length],
                         buffer, len, &error)) {
         NSData *result = [NSData dataWithBytes:buffer length:len];
        free(buffer);
        return result;
    } else {
        free(buffer);
        [PrefixerExceptions interpretErrorCode:error];
        return nil;

    }
}
@end
