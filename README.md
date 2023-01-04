# Space Command

**⚠️ Space Command is WIP. This README is written for the finished product, which does not yet exist.**

Space Command is a sync-tracker in two parts:

 - An editor/server, which provides a GUI to set up keyframes, and informs the client of keyframe updates;
 - A client library, which, either attached to the editor or consuming an exported file, provides animation/syncing values to a demo.

## Differences with Rocket

Space Command is like (and inspired by) [Rocket](https://github.com/rocket/rocket).

Space Command does some things differently to Rocket:

 - Tracks can take different data types - Rocket allows only 32-bit floats.
   - This allows 3-dimensional coordinates to be expressed in one track.
   - This allows rotations to be expressed properly as quaternions in one track, allowing proper lerping between rotations and no gimbal lock.
   - This allows colours to be expressed in one track, and to be lerped between in nicer ways (e.g. keyframes can be inputted as RGB, while lerping uses [a more suited colour space](https://bottosson.github.io/posts/oklab/))
 - More interpolation (easing) methods are available - Rocket allows only lerping between keyframes with four fixed functions.
   - Space Command allows usage of all the [CSS easing functions](https://developer.mozilla.org/en-US/docs/Web/CSS/easing-function).
   - This allows for complex effects, such as bouncing or multiple steps, without adding extra keyframes.

## Editor-Client communication

The Editor communicates with the Client over either plain TCP Sockets or Websockets. Messages are encoded using Protocol Buffers.

The Editor and the Client tell each other the track data types, easing types, commands, and 'extra features' they support during a handshake phase.

## File Format

TODO
