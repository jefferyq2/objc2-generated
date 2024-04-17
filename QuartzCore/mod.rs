// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

#![allow(unused_imports)]
#![allow(deprecated)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)]

#[link(name = "QuartzCore", kind = "framework")]
extern "C" {}

#[cfg(feature = "CAAnimation")]
#[path = "CAAnimation.rs"]
mod __CAAnimation;
#[cfg(feature = "CABase")]
#[path = "CABase.rs"]
mod __CABase;
#[cfg(feature = "CAConstraintLayoutManager")]
#[path = "CAConstraintLayoutManager.rs"]
mod __CAConstraintLayoutManager;
#[cfg(feature = "CADisplayLink")]
#[path = "CADisplayLink.rs"]
mod __CADisplayLink;
#[cfg(feature = "CAEDRMetadata")]
#[path = "CAEDRMetadata.rs"]
mod __CAEDRMetadata;
#[cfg(feature = "CAEmitterCell")]
#[path = "CAEmitterCell.rs"]
mod __CAEmitterCell;
#[cfg(feature = "CAEmitterLayer")]
#[path = "CAEmitterLayer.rs"]
mod __CAEmitterLayer;
#[cfg(feature = "CAFrameRateRange")]
#[path = "CAFrameRateRange.rs"]
mod __CAFrameRateRange;
#[cfg(feature = "CAGradientLayer")]
#[path = "CAGradientLayer.rs"]
mod __CAGradientLayer;
#[cfg(feature = "CALayer")]
#[path = "CALayer.rs"]
mod __CALayer;
#[cfg(feature = "CAMediaTiming")]
#[path = "CAMediaTiming.rs"]
mod __CAMediaTiming;
#[cfg(feature = "CAMediaTimingFunction")]
#[path = "CAMediaTimingFunction.rs"]
mod __CAMediaTimingFunction;
#[cfg(feature = "CAMetalDisplayLink")]
#[path = "CAMetalDisplayLink.rs"]
mod __CAMetalDisplayLink;
#[cfg(feature = "CAMetalLayer")]
#[path = "CAMetalLayer.rs"]
mod __CAMetalLayer;
#[cfg(feature = "CAOpenGLLayer")]
#[path = "CAOpenGLLayer.rs"]
mod __CAOpenGLLayer;
#[cfg(feature = "CARemoteLayerClient")]
#[path = "CARemoteLayerClient.rs"]
mod __CARemoteLayerClient;
#[cfg(feature = "CARemoteLayerServer")]
#[path = "CARemoteLayerServer.rs"]
mod __CARemoteLayerServer;
#[cfg(feature = "CARenderer")]
#[path = "CARenderer.rs"]
mod __CARenderer;
#[cfg(feature = "CAReplicatorLayer")]
#[path = "CAReplicatorLayer.rs"]
mod __CAReplicatorLayer;
#[cfg(feature = "CAScrollLayer")]
#[path = "CAScrollLayer.rs"]
mod __CAScrollLayer;
#[cfg(feature = "CAShapeLayer")]
#[path = "CAShapeLayer.rs"]
mod __CAShapeLayer;
#[cfg(feature = "CATextLayer")]
#[path = "CATextLayer.rs"]
mod __CATextLayer;
#[cfg(feature = "CATiledLayer")]
#[path = "CATiledLayer.rs"]
mod __CATiledLayer;
#[cfg(feature = "CATransaction")]
#[path = "CATransaction.rs"]
mod __CATransaction;
#[cfg(feature = "CATransform3D")]
#[path = "CATransform3D.rs"]
mod __CATransform3D;
#[cfg(feature = "CATransformLayer")]
#[path = "CATransformLayer.rs"]
mod __CATransformLayer;
#[cfg(feature = "CAValueFunction")]
#[path = "CAValueFunction.rs"]
mod __CAValueFunction;
#[cfg(feature = "CoreAnimation")]
#[path = "CoreAnimation.rs"]
mod __CoreAnimation;
#[cfg(feature = "CoreImage")]
#[path = "CoreImage.rs"]
mod __CoreImage;
#[cfg(feature = "CoreVideo")]
#[path = "CoreVideo.rs"]
mod __CoreVideo;

#[cfg(feature = "CAAnimation")]
pub use self::__CAAnimation::kCAAnimationCubic;
#[cfg(feature = "CAAnimation")]
pub use self::__CAAnimation::kCAAnimationCubicPaced;
#[cfg(feature = "CAAnimation")]
pub use self::__CAAnimation::kCAAnimationDiscrete;
#[cfg(feature = "CAAnimation")]
pub use self::__CAAnimation::kCAAnimationLinear;
#[cfg(feature = "CAAnimation")]
pub use self::__CAAnimation::kCAAnimationPaced;
#[cfg(feature = "CAAnimation")]
pub use self::__CAAnimation::kCAAnimationRotateAuto;
#[cfg(feature = "CAAnimation")]
pub use self::__CAAnimation::kCAAnimationRotateAutoReverse;
#[cfg(feature = "CAAnimation")]
pub use self::__CAAnimation::kCATransitionFade;
#[cfg(feature = "CAAnimation")]
pub use self::__CAAnimation::kCATransitionFromBottom;
#[cfg(feature = "CAAnimation")]
pub use self::__CAAnimation::kCATransitionFromLeft;
#[cfg(feature = "CAAnimation")]
pub use self::__CAAnimation::kCATransitionFromRight;
#[cfg(feature = "CAAnimation")]
pub use self::__CAAnimation::kCATransitionFromTop;
#[cfg(feature = "CAAnimation")]
pub use self::__CAAnimation::kCATransitionMoveIn;
#[cfg(feature = "CAAnimation")]
pub use self::__CAAnimation::kCATransitionPush;
#[cfg(feature = "CAAnimation")]
pub use self::__CAAnimation::kCATransitionReveal;
#[cfg(feature = "CAAnimation")]
pub use self::__CAAnimation::CAAnimation;
#[cfg(feature = "CAAnimation")]
pub use self::__CAAnimation::CAAnimationCalculationMode;
#[cfg(feature = "CAAnimation")]
pub use self::__CAAnimation::CAAnimationDelegate;
#[cfg(feature = "CAAnimation")]
pub use self::__CAAnimation::CAAnimationGroup;
#[cfg(feature = "CAAnimation")]
pub use self::__CAAnimation::CAAnimationRotationMode;
#[cfg(feature = "CAAnimation")]
pub use self::__CAAnimation::CABasicAnimation;
#[cfg(feature = "CAAnimation")]
pub use self::__CAAnimation::CAKeyframeAnimation;
#[cfg(feature = "CAAnimation")]
pub use self::__CAAnimation::CAPropertyAnimation;
#[cfg(feature = "CAAnimation")]
pub use self::__CAAnimation::CASpringAnimation;
#[cfg(feature = "CAAnimation")]
pub use self::__CAAnimation::CATransition;
#[cfg(feature = "CAAnimation")]
pub use self::__CAAnimation::CATransitionSubtype;
#[cfg(feature = "CAAnimation")]
pub use self::__CAAnimation::CATransitionType;
#[cfg(feature = "CABase")]
pub use self::__CABase::CACurrentMediaTime;
#[cfg(feature = "CAConstraintLayoutManager")]
pub use self::__CAConstraintLayoutManager::CAConstraint;
#[cfg(feature = "CAConstraintLayoutManager")]
pub use self::__CAConstraintLayoutManager::CAConstraintAttribute;
#[cfg(feature = "CAConstraintLayoutManager")]
pub use self::__CAConstraintLayoutManager::CAConstraintLayoutManager;
#[cfg(feature = "CADisplayLink")]
pub use self::__CADisplayLink::CADisplayLink;
#[cfg(feature = "CAEDRMetadata")]
pub use self::__CAEDRMetadata::CAEDRMetadata;
#[cfg(feature = "CAEmitterCell")]
pub use self::__CAEmitterCell::CAEmitterCell;
#[cfg(feature = "CAEmitterLayer")]
pub use self::__CAEmitterLayer::kCAEmitterLayerAdditive;
#[cfg(feature = "CAEmitterLayer")]
pub use self::__CAEmitterLayer::kCAEmitterLayerBackToFront;
#[cfg(feature = "CAEmitterLayer")]
pub use self::__CAEmitterLayer::kCAEmitterLayerCircle;
#[cfg(feature = "CAEmitterLayer")]
pub use self::__CAEmitterLayer::kCAEmitterLayerCuboid;
#[cfg(feature = "CAEmitterLayer")]
pub use self::__CAEmitterLayer::kCAEmitterLayerLine;
#[cfg(feature = "CAEmitterLayer")]
pub use self::__CAEmitterLayer::kCAEmitterLayerOldestFirst;
#[cfg(feature = "CAEmitterLayer")]
pub use self::__CAEmitterLayer::kCAEmitterLayerOldestLast;
#[cfg(feature = "CAEmitterLayer")]
pub use self::__CAEmitterLayer::kCAEmitterLayerOutline;
#[cfg(feature = "CAEmitterLayer")]
pub use self::__CAEmitterLayer::kCAEmitterLayerPoint;
#[cfg(feature = "CAEmitterLayer")]
pub use self::__CAEmitterLayer::kCAEmitterLayerPoints;
#[cfg(feature = "CAEmitterLayer")]
pub use self::__CAEmitterLayer::kCAEmitterLayerRectangle;
#[cfg(feature = "CAEmitterLayer")]
pub use self::__CAEmitterLayer::kCAEmitterLayerSphere;
#[cfg(feature = "CAEmitterLayer")]
pub use self::__CAEmitterLayer::kCAEmitterLayerSurface;
#[cfg(feature = "CAEmitterLayer")]
pub use self::__CAEmitterLayer::kCAEmitterLayerUnordered;
#[cfg(feature = "CAEmitterLayer")]
pub use self::__CAEmitterLayer::kCAEmitterLayerVolume;
#[cfg(all(feature = "CAEmitterLayer", feature = "CALayer"))]
pub use self::__CAEmitterLayer::CAEmitterLayer;
#[cfg(feature = "CAEmitterLayer")]
pub use self::__CAEmitterLayer::CAEmitterLayerEmitterMode;
#[cfg(feature = "CAEmitterLayer")]
pub use self::__CAEmitterLayer::CAEmitterLayerEmitterShape;
#[cfg(feature = "CAEmitterLayer")]
pub use self::__CAEmitterLayer::CAEmitterLayerRenderMode;
#[cfg(feature = "CAFrameRateRange")]
pub use self::__CAFrameRateRange::CAFrameRateRange;
#[cfg(feature = "CAFrameRateRange")]
pub use self::__CAFrameRateRange::CAFrameRateRangeDefault;
#[cfg(feature = "CAFrameRateRange")]
pub use self::__CAFrameRateRange::CAFrameRateRangeIsEqualToRange;
#[cfg(feature = "CAFrameRateRange")]
pub use self::__CAFrameRateRange::CAFrameRateRangeMake;
#[cfg(feature = "CAGradientLayer")]
pub use self::__CAGradientLayer::kCAGradientLayerAxial;
#[cfg(feature = "CAGradientLayer")]
pub use self::__CAGradientLayer::kCAGradientLayerConic;
#[cfg(feature = "CAGradientLayer")]
pub use self::__CAGradientLayer::kCAGradientLayerRadial;
#[cfg(all(feature = "CAGradientLayer", feature = "CALayer"))]
pub use self::__CAGradientLayer::CAGradientLayer;
#[cfg(feature = "CAGradientLayer")]
pub use self::__CAGradientLayer::CAGradientLayerType;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::kCAContentsFormatGray8Uint;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::kCAContentsFormatRGBA16Float;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::kCAContentsFormatRGBA8Uint;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::kCACornerCurveCircular;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::kCACornerCurveContinuous;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::kCAFilterLinear;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::kCAFilterNearest;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::kCAFilterTrilinear;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::kCAGravityBottom;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::kCAGravityBottomLeft;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::kCAGravityBottomRight;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::kCAGravityCenter;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::kCAGravityLeft;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::kCAGravityResize;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::kCAGravityResizeAspect;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::kCAGravityResizeAspectFill;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::kCAGravityRight;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::kCAGravityTop;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::kCAGravityTopLeft;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::kCAGravityTopRight;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::kCAOnOrderIn;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::kCAOnOrderOut;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::kCATransition;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::CAAction;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::CAAutoresizingMask;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::CACornerMask;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::CAEdgeAntialiasingMask;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::CALayer;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::CALayerContentsFilter;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::CALayerContentsFormat;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::CALayerContentsGravity;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::CALayerCornerCurve;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::CALayerDelegate;
#[cfg(feature = "CALayer")]
pub use self::__CALayer::CALayoutManager;
#[cfg(feature = "CAMediaTiming")]
pub use self::__CAMediaTiming::kCAFillModeBackwards;
#[cfg(feature = "CAMediaTiming")]
pub use self::__CAMediaTiming::kCAFillModeBoth;
#[cfg(feature = "CAMediaTiming")]
pub use self::__CAMediaTiming::kCAFillModeForwards;
#[cfg(feature = "CAMediaTiming")]
pub use self::__CAMediaTiming::kCAFillModeRemoved;
#[cfg(feature = "CAMediaTiming")]
pub use self::__CAMediaTiming::CAMediaTiming;
#[cfg(feature = "CAMediaTiming")]
pub use self::__CAMediaTiming::CAMediaTimingFillMode;
#[cfg(feature = "CAMediaTimingFunction")]
pub use self::__CAMediaTimingFunction::kCAMediaTimingFunctionDefault;
#[cfg(feature = "CAMediaTimingFunction")]
pub use self::__CAMediaTimingFunction::kCAMediaTimingFunctionEaseIn;
#[cfg(feature = "CAMediaTimingFunction")]
pub use self::__CAMediaTimingFunction::kCAMediaTimingFunctionEaseInEaseOut;
#[cfg(feature = "CAMediaTimingFunction")]
pub use self::__CAMediaTimingFunction::kCAMediaTimingFunctionEaseOut;
#[cfg(feature = "CAMediaTimingFunction")]
pub use self::__CAMediaTimingFunction::kCAMediaTimingFunctionLinear;
#[cfg(feature = "CAMediaTimingFunction")]
pub use self::__CAMediaTimingFunction::CAMediaTimingFunction;
#[cfg(feature = "CAMediaTimingFunction")]
pub use self::__CAMediaTimingFunction::CAMediaTimingFunctionName;
#[cfg(feature = "CAMetalDisplayLink")]
pub use self::__CAMetalDisplayLink::CAMetalDisplayLink;
#[cfg(feature = "CAMetalDisplayLink")]
pub use self::__CAMetalDisplayLink::CAMetalDisplayLinkDelegate;
#[cfg(feature = "CAMetalDisplayLink")]
pub use self::__CAMetalDisplayLink::CAMetalDisplayLinkUpdate;
#[cfg(feature = "CARemoteLayerClient")]
pub use self::__CARemoteLayerClient::CARemoteLayerClient;
#[cfg(feature = "CARemoteLayerServer")]
pub use self::__CARemoteLayerServer::CARemoteLayerServer;
#[cfg(feature = "CARenderer")]
pub use self::__CARenderer::kCARendererColorSpace;
#[cfg(feature = "CARenderer")]
pub use self::__CARenderer::kCARendererMetalCommandQueue;
#[cfg(feature = "CARenderer")]
pub use self::__CARenderer::CARenderer;
#[cfg(all(feature = "CALayer", feature = "CAReplicatorLayer"))]
pub use self::__CAReplicatorLayer::CAReplicatorLayer;
#[cfg(feature = "CAScrollLayer")]
pub use self::__CAScrollLayer::kCAScrollBoth;
#[cfg(feature = "CAScrollLayer")]
pub use self::__CAScrollLayer::kCAScrollHorizontally;
#[cfg(feature = "CAScrollLayer")]
pub use self::__CAScrollLayer::kCAScrollNone;
#[cfg(feature = "CAScrollLayer")]
pub use self::__CAScrollLayer::kCAScrollVertically;
#[cfg(all(feature = "CALayer", feature = "CAScrollLayer"))]
pub use self::__CAScrollLayer::CAScrollLayer;
#[cfg(feature = "CAScrollLayer")]
pub use self::__CAScrollLayer::CAScrollLayerScrollMode;
#[cfg(feature = "CAShapeLayer")]
pub use self::__CAShapeLayer::kCAFillRuleEvenOdd;
#[cfg(feature = "CAShapeLayer")]
pub use self::__CAShapeLayer::kCAFillRuleNonZero;
#[cfg(feature = "CAShapeLayer")]
pub use self::__CAShapeLayer::kCALineCapButt;
#[cfg(feature = "CAShapeLayer")]
pub use self::__CAShapeLayer::kCALineCapRound;
#[cfg(feature = "CAShapeLayer")]
pub use self::__CAShapeLayer::kCALineCapSquare;
#[cfg(feature = "CAShapeLayer")]
pub use self::__CAShapeLayer::kCALineJoinBevel;
#[cfg(feature = "CAShapeLayer")]
pub use self::__CAShapeLayer::kCALineJoinMiter;
#[cfg(feature = "CAShapeLayer")]
pub use self::__CAShapeLayer::kCALineJoinRound;
#[cfg(all(feature = "CALayer", feature = "CAShapeLayer"))]
pub use self::__CAShapeLayer::CAShapeLayer;
#[cfg(feature = "CAShapeLayer")]
pub use self::__CAShapeLayer::CAShapeLayerFillRule;
#[cfg(feature = "CAShapeLayer")]
pub use self::__CAShapeLayer::CAShapeLayerLineCap;
#[cfg(feature = "CAShapeLayer")]
pub use self::__CAShapeLayer::CAShapeLayerLineJoin;
#[cfg(feature = "CATextLayer")]
pub use self::__CATextLayer::kCAAlignmentCenter;
#[cfg(feature = "CATextLayer")]
pub use self::__CATextLayer::kCAAlignmentJustified;
#[cfg(feature = "CATextLayer")]
pub use self::__CATextLayer::kCAAlignmentLeft;
#[cfg(feature = "CATextLayer")]
pub use self::__CATextLayer::kCAAlignmentNatural;
#[cfg(feature = "CATextLayer")]
pub use self::__CATextLayer::kCAAlignmentRight;
#[cfg(feature = "CATextLayer")]
pub use self::__CATextLayer::kCATruncationEnd;
#[cfg(feature = "CATextLayer")]
pub use self::__CATextLayer::kCATruncationMiddle;
#[cfg(feature = "CATextLayer")]
pub use self::__CATextLayer::kCATruncationNone;
#[cfg(feature = "CATextLayer")]
pub use self::__CATextLayer::kCATruncationStart;
#[cfg(all(feature = "CALayer", feature = "CATextLayer"))]
pub use self::__CATextLayer::CATextLayer;
#[cfg(feature = "CATextLayer")]
pub use self::__CATextLayer::CATextLayerAlignmentMode;
#[cfg(feature = "CATextLayer")]
pub use self::__CATextLayer::CATextLayerTruncationMode;
#[cfg(all(feature = "CALayer", feature = "CATiledLayer"))]
pub use self::__CATiledLayer::CATiledLayer;
#[cfg(feature = "CATransaction")]
pub use self::__CATransaction::kCATransactionAnimationDuration;
#[cfg(feature = "CATransaction")]
pub use self::__CATransaction::kCATransactionAnimationTimingFunction;
#[cfg(feature = "CATransaction")]
pub use self::__CATransaction::kCATransactionCompletionBlock;
#[cfg(feature = "CATransaction")]
pub use self::__CATransaction::kCATransactionDisableActions;
#[cfg(feature = "CATransaction")]
pub use self::__CATransaction::CATransaction;
#[cfg(feature = "CATransform3D")]
pub use self::__CATransform3D::CATransform3D;
#[cfg(feature = "CATransform3D")]
pub use self::__CATransform3D::CATransform3DConcat;
#[cfg(feature = "CATransform3D")]
pub use self::__CATransform3D::CATransform3DEqualToTransform;
#[cfg(feature = "CATransform3D")]
pub use self::__CATransform3D::CATransform3DIdentity;
#[cfg(feature = "CATransform3D")]
pub use self::__CATransform3D::CATransform3DInvert;
#[cfg(feature = "CATransform3D")]
pub use self::__CATransform3D::CATransform3DIsAffine;
#[cfg(feature = "CATransform3D")]
pub use self::__CATransform3D::CATransform3DIsIdentity;
#[cfg(feature = "CATransform3D")]
pub use self::__CATransform3D::CATransform3DMakeRotation;
#[cfg(feature = "CATransform3D")]
pub use self::__CATransform3D::CATransform3DMakeScale;
#[cfg(feature = "CATransform3D")]
pub use self::__CATransform3D::CATransform3DMakeTranslation;
#[cfg(feature = "CATransform3D")]
pub use self::__CATransform3D::CATransform3DRotate;
#[cfg(feature = "CATransform3D")]
pub use self::__CATransform3D::CATransform3DScale;
#[cfg(feature = "CATransform3D")]
pub use self::__CATransform3D::CATransform3DTranslate;
#[cfg(feature = "CATransform3D")]
pub use self::__CATransform3D::NSValueCATransform3DAdditions;
#[cfg(all(feature = "CALayer", feature = "CATransformLayer"))]
pub use self::__CATransformLayer::CATransformLayer;
#[cfg(feature = "CAValueFunction")]
pub use self::__CAValueFunction::kCAValueFunctionRotateX;
#[cfg(feature = "CAValueFunction")]
pub use self::__CAValueFunction::kCAValueFunctionRotateY;
#[cfg(feature = "CAValueFunction")]
pub use self::__CAValueFunction::kCAValueFunctionRotateZ;
#[cfg(feature = "CAValueFunction")]
pub use self::__CAValueFunction::kCAValueFunctionScale;
#[cfg(feature = "CAValueFunction")]
pub use self::__CAValueFunction::kCAValueFunctionScaleX;
#[cfg(feature = "CAValueFunction")]
pub use self::__CAValueFunction::kCAValueFunctionScaleY;
#[cfg(feature = "CAValueFunction")]
pub use self::__CAValueFunction::kCAValueFunctionScaleZ;
#[cfg(feature = "CAValueFunction")]
pub use self::__CAValueFunction::kCAValueFunctionTranslate;
#[cfg(feature = "CAValueFunction")]
pub use self::__CAValueFunction::kCAValueFunctionTranslateX;
#[cfg(feature = "CAValueFunction")]
pub use self::__CAValueFunction::kCAValueFunctionTranslateY;
#[cfg(feature = "CAValueFunction")]
pub use self::__CAValueFunction::kCAValueFunctionTranslateZ;
#[cfg(feature = "CAValueFunction")]
pub use self::__CAValueFunction::CAValueFunction;
#[cfg(feature = "CAValueFunction")]
pub use self::__CAValueFunction::CAValueFunctionName;
