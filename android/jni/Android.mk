# Copyright (C) 2010 The Android Open Source Project
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
LOCAL_PATH := $(call my-dir)

include $(CLEAR_VARS)

NDK_MODULE_PATH := /usr/local/Cellar/android-ndk/

LOCAL_MODULE    := rust-prebuilt
LOCAL_SRC_FILES := ../../inrustwetrust/target/arm-linux-androideabi/debug/libaxolotl.a

include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)

LOCAL_MODULE    := axolotl
#LOCAL_SRC_FILES := test.c

LOCAL_LDLIBS    := -ldl -llog -lgcc -lc -lm
LOCAL_STATIC_LIBRARIES += rust-prebuilt

include $(BUILD_SHARED_LIBRARY)

#$(call import-module)
