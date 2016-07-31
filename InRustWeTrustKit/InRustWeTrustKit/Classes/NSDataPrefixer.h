//
//  NSDataPrefixer.h
//  Pods
//
//  Created by geal on 31/07/16.
//
//

#import <Foundation/Foundation.h>
#import "inrustwetrust.h"

@interface NSDataPrefixer : NSObject
@property (assign) Prefixer* prefixer;

-(instancetype)initWithData:(NSData*) data;
-(NSData*)prefixData:(NSData*) data;

@end
