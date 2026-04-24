//! Rust FFI bindings for the macOS AudioUnit v2 (AUv2) C API.
//!
//! These are hand-written bindings against the stable macOS AudioUnit C API.
//! The structs and constants are derived from:
//!   - AudioToolbox/AUComponent.h
//!   - AudioToolbox/AudioUnitProperties.h
//!   - AudioToolbox/AudioComponent.h
//!   - CoreAudioTypes/CoreAudioBaseTypes.h
//!
//! This crate is macOS-only.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]
#![cfg(target_os = "macos")]

use std::ffi::c_void;
use std::os::raw::{c_char, c_int};

// ─── Primitive type aliases ────────────────────────────────────────────────────

pub type OSStatus = i32;
pub type UInt8 = u8;
pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;
pub type SInt16 = i16;
pub type SInt32 = i32;
pub type Float32 = f32;
pub type Float64 = f64;
pub type Boolean = u8;

/// An opaque type for an AudioComponent instance (i.e. an AudioUnit).
pub enum OpaqueAudioComponentInstance {}
pub type AudioComponentInstance = *mut OpaqueAudioComponentInstance;
pub type AudioUnit = AudioComponentInstance;

// ─── AudioComponent / PlugIn interface ────────────────────────────────────────

/// Prototype for every method dispatched through [`AudioComponentPlugInInterface::Lookup`].
pub type AudioComponentMethod = unsafe extern "C" fn(*mut c_void, ...) -> OSStatus;

/// The vtable that every AUv2 plugin must provide.
///
/// The factory function allocates this struct (embedded in the plugin instance)
/// and returns a pointer to it.  `Open` / `Close` manage the instance lifetime;
/// `Lookup` returns the method pointer for a given selector.
#[repr(C)]
pub struct AudioComponentPlugInInterface {
    /// Called by the host shortly after the factory function returns.
    /// `self_ptr` is the same pointer returned by the factory.
    pub Open: unsafe extern "C" fn(self_ptr: *mut c_void, instance: AudioUnit) -> OSStatus,
    /// Called to destroy the plugin instance.
    pub Close: unsafe extern "C" fn(self_ptr: *mut c_void) -> OSStatus,
    /// Returns a method pointer for `selector`, or null if not supported.
    pub Lookup: unsafe extern "C" fn(selector: SInt16) -> Option<AudioComponentMethod>,
    /// Must be null.
    pub reserved: *mut c_void,
}

/// Identifies an Audio Component (type + subtype + manufacturer).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct AudioComponentDescription {
    pub componentType: UInt32,
    pub componentSubType: UInt32,
    pub componentManufacturer: UInt32,
    pub componentFlags: UInt32,
    pub componentFlagsMask: UInt32,
}

// ─── Audio Unit type four-char codes ──────────────────────────────────────────

pub const kAudioUnitType_Output: UInt32 = 0x61756F75; // 'auou'
pub const kAudioUnitType_MusicDevice: UInt32 = 0x61756D75; // 'aumu'
pub const kAudioUnitType_MusicEffect: UInt32 = 0x61756D66; // 'aumf'
pub const kAudioUnitType_FormatConverter: UInt32 = 0x61756663; // 'aufc'
pub const kAudioUnitType_Effect: UInt32 = 0x61756678; // 'aufx'
pub const kAudioUnitType_Mixer: UInt32 = 0x61756D78; // 'aumx'
pub const kAudioUnitType_Panner: UInt32 = 0x6175706E; // 'aupn'
pub const kAudioUnitType_Generator: UInt32 = 0x6175676E; // 'augn'
pub const kAudioUnitType_OfflineEffect: UInt32 = 0x61756F6C; // 'auol'

// ─── Selector constants (AudioUnitRange) ──────────────────────────────────────

pub const kAudioUnitInitializeSelect: SInt16 = 0x0001;
pub const kAudioUnitUninitializeSelect: SInt16 = 0x0002;
pub const kAudioUnitGetPropertyInfoSelect: SInt16 = 0x0003;
pub const kAudioUnitGetPropertySelect: SInt16 = 0x0004;
pub const kAudioUnitSetPropertySelect: SInt16 = 0x0005;
pub const kAudioUnitGetParameterSelect: SInt16 = 0x0006;
pub const kAudioUnitSetParameterSelect: SInt16 = 0x0007;
pub const kAudioUnitResetSelect: SInt16 = 0x0009;
pub const kAudioUnitAddPropertyListenerSelect: SInt16 = 0x000A;
pub const kAudioUnitRemovePropertyListenerSelect: SInt16 = 0x000B;
pub const kAudioUnitRenderSelect: SInt16 = 0x000E;
pub const kAudioUnitAddRenderNotifySelect: SInt16 = 0x000F;
pub const kAudioUnitRemoveRenderNotifySelect: SInt16 = 0x0010;
pub const kAudioUnitScheduleParametersSelect: SInt16 = 0x0011;
pub const kAudioUnitRemovePropertyListenerWithUserDataSelect: SInt16 = 0x0012;

// ─── Dispatch function prototypes ─────────────────────────────────────────────

pub type AudioUnitPropertyID = UInt32;
pub type AudioUnitScope = UInt32;
pub type AudioUnitElement = UInt32;
pub type AudioUnitParameterID = UInt32;
pub type AudioUnitParameterValue = Float32;
pub type AudioUnitRenderActionFlags = UInt32;

pub type AudioUnitInitializeProc =
    unsafe extern "C" fn(self_ptr: *mut c_void) -> OSStatus;

pub type AudioUnitUninitializeProc =
    unsafe extern "C" fn(self_ptr: *mut c_void) -> OSStatus;

pub type AudioUnitGetPropertyInfoProc = unsafe extern "C" fn(
    self_ptr: *mut c_void,
    inID: AudioUnitPropertyID,
    inScope: AudioUnitScope,
    inElement: AudioUnitElement,
    outDataSize: *mut UInt32,
    outWritable: *mut Boolean,
) -> OSStatus;

pub type AudioUnitGetPropertyProc = unsafe extern "C" fn(
    self_ptr: *mut c_void,
    inID: AudioUnitPropertyID,
    inScope: AudioUnitScope,
    inElement: AudioUnitElement,
    outData: *mut c_void,
    ioDataSize: *mut UInt32,
) -> OSStatus;

pub type AudioUnitSetPropertyProc = unsafe extern "C" fn(
    self_ptr: *mut c_void,
    inID: AudioUnitPropertyID,
    inScope: AudioUnitScope,
    inElement: AudioUnitElement,
    inData: *const c_void,
    inDataSize: UInt32,
) -> OSStatus;

pub type AudioUnitGetParameterProc = unsafe extern "C" fn(
    self_ptr: *mut c_void,
    inID: AudioUnitParameterID,
    inScope: AudioUnitScope,
    inElement: AudioUnitElement,
    outValue: *mut AudioUnitParameterValue,
) -> OSStatus;

pub type AudioUnitSetParameterProc = unsafe extern "C" fn(
    self_ptr: *mut c_void,
    inID: AudioUnitParameterID,
    inScope: AudioUnitScope,
    inElement: AudioUnitElement,
    inValue: AudioUnitParameterValue,
    inBufferOffsetInFrames: UInt32,
) -> OSStatus;

pub type AudioUnitRenderProc = unsafe extern "C" fn(
    self_ptr: *mut c_void,
    ioActionFlags: *mut AudioUnitRenderActionFlags,
    inTimeStamp: *const AudioTimeStamp,
    inOutputBusNumber: UInt32,
    inNumberFrames: UInt32,
    ioData: *mut AudioBufferList,
) -> OSStatus;

pub type AudioUnitResetProc = unsafe extern "C" fn(
    self_ptr: *mut c_void,
    inScope: AudioUnitScope,
    inElement: AudioUnitElement,
) -> OSStatus;

pub type AudioUnitAddPropertyListenerProc = unsafe extern "C" fn(
    self_ptr: *mut c_void,
    inID: AudioUnitPropertyID,
    inProc: AudioUnitPropertyListenerProc,
    inProcUserData: *mut c_void,
) -> OSStatus;

pub type AudioUnitRemovePropertyListenerWithUserDataProc = unsafe extern "C" fn(
    self_ptr: *mut c_void,
    inID: AudioUnitPropertyID,
    inProc: AudioUnitPropertyListenerProc,
    inProcUserData: *mut c_void,
) -> OSStatus;

/// Callback the host registers to be notified of property changes.
pub type AudioUnitPropertyListenerProc = unsafe extern "C" fn(
    inRefCon: *mut c_void,
    inUnit: AudioUnit,
    inID: AudioUnitPropertyID,
    inScope: AudioUnitScope,
    inElement: AudioUnitElement,
);

/// Callback the host can register to be called at the beginning/end of each render cycle.
pub type AURenderCallback = unsafe extern "C" fn(
    inRefCon: *mut c_void,
    ioActionFlags: *mut AudioUnitRenderActionFlags,
    inTimeStamp: *const AudioTimeStamp,
    inBusNumber: UInt32,
    inNumberFrames: UInt32,
    ioData: *mut AudioBufferList,
) -> OSStatus;

/// A render callback + user-data pair, used with `kAudioUnitProperty_SetRenderCallback`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AURenderCallbackStruct {
    pub inputProc: Option<AURenderCallback>,
    pub inputProcRefCon: *mut c_void,
}
unsafe impl Send for AURenderCallbackStruct {}

// ─── Render action flags ───────────────────────────────────────────────────────

pub const kAudioUnitRenderAction_PreRender: AudioUnitRenderActionFlags = 1 << 2;
pub const kAudioUnitRenderAction_PostRender: AudioUnitRenderActionFlags = 1 << 3;
pub const kAudioUnitRenderAction_OutputIsSilence: AudioUnitRenderActionFlags = 1 << 4;

// ─── OSStatus error codes ──────────────────────────────────────────────────────

pub const noErr: OSStatus = 0;
pub const kAudioUnitErr_InvalidProperty: OSStatus = -10879;
pub const kAudioUnitErr_InvalidParameter: OSStatus = -10878;
pub const kAudioUnitErr_InvalidElement: OSStatus = -10877;
pub const kAudioUnitErr_NoConnection: OSStatus = -10876;
pub const kAudioUnitErr_FailedInitialization: OSStatus = -10875;
pub const kAudioUnitErr_TooManyFramesToProcess: OSStatus = -10874;
pub const kAudioUnitErr_InvalidFile: OSStatus = -10871;
pub const kAudioUnitErr_FormatNotSupported: OSStatus = -10868;
pub const kAudioUnitErr_Uninitialized: OSStatus = -10867;
pub const kAudioUnitErr_InvalidScope: OSStatus = -10866;
pub const kAudioUnitErr_PropertyNotWritable: OSStatus = -10865;
pub const kAudioUnitErr_CannotDoInCurrentContext: OSStatus = -10863;
pub const kAudioUnitErr_InvalidPropertyValue: OSStatus = -10851;
pub const kAudioUnitErr_PropertyNotInUse: OSStatus = -10850;
pub const kAudioUnitErr_Initialized: OSStatus = -10849;
pub const kAudioUnitErr_InvalidOfflineRender: OSStatus = -10848;
pub const kAudioUnitErr_Unauthorized: OSStatus = -10847;
pub const kAudioComponentErr_InstanceInvalidated: OSStatus = -66749;

// ─── Scope constants ───────────────────────────────────────────────────────────

pub const kAudioUnitScope_Global: AudioUnitScope = 0;
pub const kAudioUnitScope_Input: AudioUnitScope = 1;
pub const kAudioUnitScope_Output: AudioUnitScope = 2;
pub const kAudioUnitScope_Group: AudioUnitScope = 3;
pub const kAudioUnitScope_Part: AudioUnitScope = 4;
pub const kAudioUnitScope_Note: AudioUnitScope = 5;

// ─── Property IDs ─────────────────────────────────────────────────────────────

pub const kAudioUnitProperty_ClassInfo: AudioUnitPropertyID = 0;
pub const kAudioUnitProperty_MakeConnection: AudioUnitPropertyID = 1;
pub const kAudioUnitProperty_SampleRate: AudioUnitPropertyID = 2;
pub const kAudioUnitProperty_ParameterList: AudioUnitPropertyID = 3;
pub const kAudioUnitProperty_ParameterInfo: AudioUnitPropertyID = 4;
pub const kAudioUnitProperty_FastDispatch: AudioUnitPropertyID = 5;
pub const kAudioUnitProperty_CPULoad: AudioUnitPropertyID = 6;
pub const kAudioUnitProperty_StreamFormat: AudioUnitPropertyID = 8;
pub const kAudioUnitProperty_ElementCount: AudioUnitPropertyID = 11;
pub const kAudioUnitProperty_Latency: AudioUnitPropertyID = 12;
pub const kAudioUnitProperty_SupportedNumChannels: AudioUnitPropertyID = 13;
pub const kAudioUnitProperty_MaximumFramesPerSlice: AudioUnitPropertyID = 14;
pub const kAudioUnitProperty_SetExternalBuffer: AudioUnitPropertyID = 15;
pub const kAudioUnitProperty_ParameterValueStrings: AudioUnitPropertyID = 16;
pub const kAudioUnitProperty_GetUIComponentList: AudioUnitPropertyID = 18;
pub const kAudioUnitProperty_AudioChannelLayout: AudioUnitPropertyID = 19;
pub const kAudioUnitProperty_TailTime: AudioUnitPropertyID = 20;
pub const kAudioUnitProperty_BypassEffect: AudioUnitPropertyID = 21;
pub const kAudioUnitProperty_LastRenderError: AudioUnitPropertyID = 22;
pub const kAudioUnitProperty_SetRenderCallback: AudioUnitPropertyID = 23;
pub const kAudioUnitProperty_FactoryPresets: AudioUnitPropertyID = 24;
pub const kAudioUnitProperty_ContextName: AudioUnitPropertyID = 25;
pub const kAudioUnitProperty_RenderQuality: AudioUnitPropertyID = 26;
pub const kAudioUnitProperty_HostCallbacks: AudioUnitPropertyID = 27;
pub const kAudioUnitProperty_InPlaceProcessing: AudioUnitPropertyID = 29;
pub const kAudioUnitProperty_ElementName: AudioUnitPropertyID = 30;
pub const kAudioUnitProperty_CocoaUI: AudioUnitPropertyID = 31;
pub const kAudioUnitProperty_SupportedChannelLayoutTags: AudioUnitPropertyID = 32;
pub const kAudioUnitProperty_ParameterStringFromValue: AudioUnitPropertyID = 33;
pub const kAudioUnitProperty_ParameterIDName: AudioUnitPropertyID = 34;
pub const kAudioUnitProperty_ParameterClumpName: AudioUnitPropertyID = 35;
pub const kAudioUnitProperty_PresentPreset: AudioUnitPropertyID = 36;
pub const kAudioUnitProperty_OfflineRender: AudioUnitPropertyID = 37;
pub const kAudioUnitProperty_ParameterValueFromString: AudioUnitPropertyID = 38;
pub const kAudioUnitProperty_IconLocation: AudioUnitPropertyID = 39;
pub const kAudioUnitProperty_PresentationLatency: AudioUnitPropertyID = 40;
pub const kAudioUnitProperty_DependentParameters: AudioUnitPropertyID = 45;
pub const kAudioUnitProperty_AUHostIdentifier: AudioUnitPropertyID = 46;
pub const kAudioUnitProperty_MIDIOutputCallbackInfo: AudioUnitPropertyID = 47;
pub const kAudioUnitProperty_MIDIOutputCallback: AudioUnitPropertyID = 48;
pub const kAudioUnitProperty_InputSamplesInOutput: AudioUnitPropertyID = 49;
pub const kAudioUnitProperty_ClassInfoFromDocument: AudioUnitPropertyID = 50;
pub const kAudioUnitProperty_ShouldAllocateBuffer: AudioUnitPropertyID = 51;
pub const kAudioUnitProperty_FrequencyResponse: AudioUnitPropertyID = 52;
pub const kAudioUnitProperty_ParameterHistoryInfo: AudioUnitPropertyID = 53;
pub const kAudioUnitProperty_NickName: AudioUnitPropertyID = 54;

// ─── AudioFormatID ─────────────────────────────────────────────────────────────

pub type AudioFormatID = UInt32;
pub type AudioFormatFlags = UInt32;

pub const kAudioFormatLinearPCM: AudioFormatID = 0x6C70636D; // 'lpcm'

pub const kAudioFormatFlagIsFloat: AudioFormatFlags = 1 << 0;
pub const kAudioFormatFlagIsBigEndian: AudioFormatFlags = 1 << 1;
pub const kAudioFormatFlagIsSignedInteger: AudioFormatFlags = 1 << 2;
pub const kAudioFormatFlagIsPacked: AudioFormatFlags = 1 << 3;
pub const kAudioFormatFlagIsAlignedHigh: AudioFormatFlags = 1 << 4;
pub const kAudioFormatFlagIsNonInterleaved: AudioFormatFlags = 1 << 5;
pub const kAudioFormatFlagIsNonMixable: AudioFormatFlags = 1 << 6;

/// Native-endian, non-interleaved, packed 32-bit float – the standard AU processing format.
pub const kAudioFormatFlagsNativeFloatPacked: AudioFormatFlags =
    kAudioFormatFlagIsFloat | kAudioFormatFlagIsPacked;

// ─── AudioStreamBasicDescription ───────────────────────────────────────────────

/// Describes a PCM audio stream format.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct AudioStreamBasicDescription {
    pub mSampleRate: Float64,
    pub mFormatID: AudioFormatID,
    pub mFormatFlags: AudioFormatFlags,
    pub mBytesPerPacket: UInt32,
    pub mFramesPerPacket: UInt32,
    pub mBytesPerFrame: UInt32,
    pub mChannelsPerFrame: UInt32,
    pub mBitsPerChannel: UInt32,
    pub mReserved: UInt32,
}

impl AudioStreamBasicDescription {
    /// Helper: build an ASBD for non-interleaved 32-bit float PCM.
    pub fn noninterleaved_float32(sample_rate: f64, num_channels: u32) -> Self {
        Self {
            mSampleRate: sample_rate,
            mFormatID: kAudioFormatLinearPCM,
            mFormatFlags: kAudioFormatFlagsNativeFloatPacked
                | kAudioFormatFlagIsNonInterleaved,
            mBytesPerPacket: 4,
            mFramesPerPacket: 1,
            mBytesPerFrame: 4,
            mChannelsPerFrame: num_channels,
            mBitsPerChannel: 32,
            mReserved: 0,
        }
    }
}

// ─── AudioBuffer / AudioBufferList ─────────────────────────────────────────────

/// A single audio buffer in a buffer list.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioBuffer {
    pub mNumberChannels: UInt32,
    pub mDataByteSize: UInt32,
    pub mData: *mut c_void,
}

unsafe impl Send for AudioBuffer {}

/// A variable-length list of [`AudioBuffer`]s.
///
/// # Layout note
/// The C struct has a fixed-size `mBuffers[1]` field; in practice the host
/// allocates enough memory for `mNumberBuffers` entries.  In Rust we represent
/// this via raw pointer arithmetic (see [`AudioBufferList::buffers`]).
#[repr(C)]
pub struct AudioBufferList {
    pub mNumberBuffers: UInt32,
    pub mBuffers: [AudioBuffer; 1], // variable-length – use buffers() accessor
}

impl AudioBufferList {
    /// Return a slice over all buffers.
    ///
    /// # Safety
    /// The caller must ensure `self` points to a valid `AudioBufferList` with
    /// `mNumberBuffers` entries allocated after the struct.
    pub unsafe fn buffers(&self) -> &[AudioBuffer] {
        std::slice::from_raw_parts(
            self.mBuffers.as_ptr(),
            self.mNumberBuffers as usize,
        )
    }

    /// Return a mutable slice over all buffers.
    ///
    /// # Safety
    /// Same as [`buffers`], plus `self` must not be aliased.
    pub unsafe fn buffers_mut(&mut self) -> &mut [AudioBuffer] {
        std::slice::from_raw_parts_mut(
            self.mBuffers.as_mut_ptr(),
            self.mNumberBuffers as usize,
        )
    }
}

// ─── SMPTETime / AudioTimeStamp ───────────────────────────────────────────────

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SMPTETime {
    pub mSubframes: SInt16,
    pub mSubframeDivisor: SInt16,
    pub mCounter: UInt32,
    pub mType: UInt32,
    pub mFlags: UInt32,
    pub mHours: SInt16,
    pub mMinutes: SInt16,
    pub mSeconds: SInt16,
    pub mFrames: SInt16,
}

pub const kAudioTimeStampSampleTimeValid: UInt32 = 1 << 0;
pub const kAudioTimeStampHostTimeValid: UInt32 = 1 << 1;
pub const kAudioTimeStampRateScalarValid: UInt32 = 1 << 2;

/// Represents a point in time in several coordinate systems.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct AudioTimeStamp {
    pub mSampleTime: Float64,
    pub mHostTime: UInt64,
    pub mRateScalar: Float64,
    pub mWordClockTime: UInt64,
    pub mSMPTETime: SMPTETime,
    pub mFlags: UInt32,
    pub mReserved: UInt32,
}

// ─── AudioUnitParameterInfo ────────────────────────────────────────────────────

pub type CFStringRef = *mut c_void; // opaque CoreFoundation type

pub type AudioUnitParameterUnit = UInt32;
pub type AudioUnitParameterOptions = UInt32;

/// Metadata about one plugin parameter, returned via `kAudioUnitProperty_ParameterInfo`.
#[repr(C)]
pub struct AudioUnitParameterInfo {
    /// Deprecated legacy C-string name (set to all-zeros).
    pub name: [c_char; 52],
    /// Valid when `kAudioUnitParameterFlag_HasCFNameString` is set.
    pub cfNameString: CFStringRef,
    pub unit: AudioUnitParameterUnit,
    pub minValue: AudioUnitParameterValue,
    pub maxValue: AudioUnitParameterValue,
    pub defaultValue: AudioUnitParameterValue,
    pub flags: AudioUnitParameterOptions,
}

// Parameter unit constants
pub const kAudioUnitParameterUnit_Generic: AudioUnitParameterUnit = 0;
pub const kAudioUnitParameterUnit_Indexed: AudioUnitParameterUnit = 1;
pub const kAudioUnitParameterUnit_Boolean: AudioUnitParameterUnit = 2;
pub const kAudioUnitParameterUnit_Percent: AudioUnitParameterUnit = 3;
pub const kAudioUnitParameterUnit_Seconds: AudioUnitParameterUnit = 4;
pub const kAudioUnitParameterUnit_Hertz: AudioUnitParameterUnit = 8;
pub const kAudioUnitParameterUnit_Decibels: AudioUnitParameterUnit = 13;
pub const kAudioUnitParameterUnit_LinearGain: AudioUnitParameterUnit = 14;
pub const kAudioUnitParameterUnit_CustomUnit: AudioUnitParameterUnit = 26;

// Parameter option flags
pub const kAudioUnitParameterFlag_CFNameRelease: AudioUnitParameterOptions = 1 << 4;
pub const kAudioUnitParameterFlag_HasCFNameString: AudioUnitParameterOptions = 1 << 27;
pub const kAudioUnitParameterFlag_IsReadable: AudioUnitParameterOptions = 1 << 30;
pub const kAudioUnitParameterFlag_IsWritable: AudioUnitParameterOptions = 1 << 31;
pub const kAudioUnitParameterFlag_IsHighResolution: AudioUnitParameterOptions = 1 << 23;
pub const kAudioUnitParameterFlag_CanRamp: AudioUnitParameterOptions = 1 << 25;
pub const kAudioUnitParameterFlag_IsGlobalMeta: AudioUnitParameterOptions = 1 << 28;
pub const kAudioUnitParameterFlag_ValuesHaveStrings: AudioUnitParameterOptions = 1 << 21;
pub const kAudioUnitParameterFlag_DisplayLogarithmic: AudioUnitParameterOptions = 1 << 22;
pub const kAudioUnitParameterFlag_NonRealTime: AudioUnitParameterOptions = 1 << 24;
pub const kAudioUnitParameterFlag_ExpertMode: AudioUnitParameterOptions = 1 << 26;
pub const kAudioUnitParameterFlag_OmitFromPresets: AudioUnitParameterOptions = 1 << 13;

// ─── AUPreset ─────────────────────────────────────────────────────────────────

/// A factory preset entry returned by `kAudioUnitProperty_FactoryPresets`.
#[repr(C)]
pub struct AUPreset {
    pub presetNumber: SInt32,
    pub presetName: CFStringRef,
}

// ─── AUCocoaViewInfo ──────────────────────────────────────────────────────────

/// Returned by `kAudioUnitProperty_CocoaUI` to tell the host where the view
/// factory class lives.
#[repr(C)]
pub struct AUCocoaViewInfo {
    /// A `CFBundleRef` for the bundle that contains the view.
    pub mCocoaAUViewBundleLocation: CFStringRef, // actually CFURLRef, but opaque here
    /// A `CFStringRef` naming the `AUCocoaUIBase` subclass in that bundle.
    pub mCocoaAUViewClass: [CFStringRef; 1], // variable length array of 1+
}

// ─── Supported channel info ────────────────────────────────────────────────────

/// One entry in the array returned by `kAudioUnitProperty_SupportedNumChannels`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AUChannelInfo {
    /// Number of input channels (-1 = any, -2 = any matching output channel count).
    pub inChannels: SInt16,
    /// Number of output channels (-1 = any).
    pub outChannels: SInt16,
}

// ─── Host callback info ────────────────────────────────────────────────────────

/// Host callbacks registered via `kAudioUnitProperty_HostCallbacks`.
#[repr(C)]
pub struct HostCallbackInfo {
    pub hostUserData: *mut c_void,
    pub beatAndTempoProc: Option<
        unsafe extern "C" fn(
            inHostUserData: *mut c_void,
            outCurrentBeat: *mut Float64,
            outCurrentTempo: *mut Float64,
        ) -> OSStatus,
    >,
    pub musicalTimeLocationProc: Option<
        unsafe extern "C" fn(
            inHostUserData: *mut c_void,
            outDeltaSampleOffsetToNextBeat: *mut UInt32,
            outTimeSig_Numerator: *mut Float32,
            outTimeSig_Denominator: *mut UInt32,
            outCurrentMeasureDownBeat: *mut Float64,
        ) -> OSStatus,
    >,
    pub transportStateProc: Option<
        unsafe extern "C" fn(
            inHostUserData: *mut c_void,
            outIsPlaying: *mut Boolean,
            outTransportStateChanged: *mut Boolean,
            outCurrentSampleInTimeLine: *mut Float64,
            outIsCycling: *mut Boolean,
            outCycleStartBeat: *mut Float64,
            outCycleEndBeat: *mut Float64,
        ) -> OSStatus,
    >,
    pub transportStateProc2: Option<
        unsafe extern "C" fn(
            inHostUserData: *mut c_void,
            outIsPlaying: *mut Boolean,
            outIsRecording: *mut Boolean,
            outTransportStateChanged: *mut Boolean,
            outCurrentSampleInTimeLine: *mut Float64,
            outIsCycling: *mut Boolean,
            outCycleStartBeat: *mut Float64,
            outCycleEndBeat: *mut Float64,
        ) -> OSStatus,
    >,
}

unsafe impl Send for HostCallbackInfo {}
unsafe impl Sync for HostCallbackInfo {}

// ─── CoreFoundation helpers (minimal) ─────────────────────────────────────────

/// Create a `CFStringRef` from a Rust `&str` (UTF-8 → UTF-16).
/// Returns null on failure.
///
/// # Safety
/// The caller is responsible for calling `CFRelease` on the returned string.
pub unsafe fn cf_string_create(s: &str) -> CFStringRef {
    extern "C" {
        fn CFStringCreateWithBytes(
            alloc: *mut c_void,
            bytes: *const u8,
            numBytes: c_int,
            encoding: UInt32,
            isExternalRepresentation: Boolean,
        ) -> CFStringRef;
    }
    // kCFStringEncodingUTF8 = 0x08000100
    CFStringCreateWithBytes(
        std::ptr::null_mut(),
        s.as_ptr(),
        s.len() as c_int,
        0x0800_0100,
        0,
    )
}

/// Release a `CFTypeRef`.
///
/// # Safety
/// `ptr` must be a valid CoreFoundation object (or null).
pub unsafe fn cf_release(ptr: CFStringRef) {
    if !ptr.is_null() {
        extern "C" {
            fn CFRelease(cf: CFStringRef);
        }
        CFRelease(ptr);
    }
}
