import React, { Component } from "react";
import { findDOMNode } from "react-dom";
import { hot } from "react-hot-loader";
import screenfull from "screenfull";

import "./reset.css";
import "./defaults.css";
import "./range.css";
import "./App.css";

import { version } from "../../package.json";
import ReactPlayer from "../ReactPlayer";
import Duration from "./Duration";

const MULTIPLE_SOURCES = [
  {
    src: "http://clips.vorwaerts-gmbh.de/big_buck_bunny.mp4",
    type: "video/mp4"
  },
  {
    src: "http://clips.vorwaerts-gmbh.de/big_buck_bunny.ogv",
    type: "video/ogv"
  },
  {
    src: "http://clips.vorwaerts-gmbh.de/big_buck_bunny.webm",
    type: "video/webm"
  }
];

const torontoRadio1 = "http://cbc_r1_tor.akacast.akamaistream.net/7/632/451661/v1/rc.akacast.akamaistream.net/cbc_r1_tor"; 
const torontoRadio2 = "http://cbc_r2_tor.akacast.akamaistream.net/7/364/451661/v1/rc.akacast.akamaistream.net/cbc_r2_tor";
const bassdrive = "http://bassdrive.radioca.st/;stream/1";

class App extends Component {
  state = {
    url: bassdrive, //
    pip: false,
    playing: false,
    controls: false,
    light: false,
    volume: 0.8,
    muted: false,
    played: 0,
    loaded: 0,
    duration: 0,
    playbackRate: 1.0,
    loop: false,
    meta: null
  };
  componentDidMount() {}
  load = url => {
    this.setState({
      url,
      played: 0,
      loaded: 0,
      pip: false
    });
  };
  playPause = () => {
    this.setState({ playing: !this.state.playing });
  };
  stop = () => {
    this.setState({ url: null, playing: false });
  };
  toggleControls = () => {
    const url = this.state.url;
    this.setState(
      {
        controls: !this.state.controls,
        url: null
      },
      () => this.load(url)
    );
  };
  toggleLight = () => {
    this.setState({ light: !this.state.light });
  };
  toggleLoop = () => {
    this.setState({ loop: !this.state.loop });
  };
  setVolume = e => {
    this.setState({ volume: parseFloat(e.target.value) });
  };
  toggleMuted = () => {
    this.setState({ muted: !this.state.muted });
  };
  setPlaybackRate = e => {
    this.setState({ playbackRate: parseFloat(e.target.value) });
  };
  togglePIP = () => {
    this.setState({ pip: !this.state.pip });
  };
  onPlay = () => {
    console.log("onPlay");
    const self = this;

    const url = this.state.url;

    const Parser = require("icecast-parser");
    const radioStation = new Parser(url);

    // "http://bassdrive.radioca.st/;stream/1"

    radioStation.on("metadata", function(metadata) {
      console.log("meta:", metadata);
      // self.onMeta(metadata);
      console.log("title: ", metadata);

      // EW Live from NY hosted by Overfiend - special guest SOHLMAN
      //The Prague Connection June 17th 2019 - hosted by Blofeld  
      const host = metadata.StreamTitle.split("by")[1];
      console.log("TCL: App -> onPlay -> host", host)
      const show = metadata.StreamTitle.split("hosted")[0]; // need parse out date
    
      self.setState({ meta: metadata });
    });

    this.setState({ playing: true });
  };
  onEnablePIP = () => {
    console.log("onEnablePIP");
    this.setState({ pip: true });
  };
  onDisablePIP = () => {
    console.log("onDisablePIP");
    this.setState({ pip: false });
  };
  onPause = () => {
    console.log("onPause");
    this.setState({ playing: false });
  };
  onSeekMouseDown = e => {
    this.setState({ seeking: true });
  };
  onSeekChange = e => {
    this.setState({ played: parseFloat(e.target.value) });
  };
  onSeekMouseUp = e => {
    this.setState({ seeking: false });
    this.player.seekTo(parseFloat(e.target.value));
  };
  onProgress = state => {
    console.log("onProgress", state);
    // We only want to update time slider if we are not currently seeking
    if (!this.state.seeking) {
      this.setState(state);
    }
  };
  onEnded = () => {
    console.log("onEnded");
    this.setState({ playing: this.state.loop });
  };
  onDuration = duration => {
    console.log("onDuration", duration);
    this.setState({ duration });
  };
  onClickFullscreen = () => {
    screenfull.request(findDOMNode(this.player));
  };
  renderLoadButton = (url, label) => {
    return <button onClick={() => this.load(url)}>{label}</button>;
  };
  ref = player => {
    this.player = player;
  };
  renderMeta = meta => {
    if (this.state.meta) {
      return (
        <div className="meta">
          <h2>{this.state.meta.StreamTitle}</h2>
        </div>
      );
    }
  };
  render() {
    const {
      url,
      playing,
      controls,
      light,
      volume,
      muted,
      loop,
      played,
      loaded,
      duration,
      playbackRate,
      pip
    } = this.state;
    const SEPARATOR = " · ";

    return (
      <div className="app">
        <section className="section">
          <h1>Bassdrive Radio</h1>
          <div className="player-wrapper" style={{display: "none"}}>
            <ReactPlayer
              ref={this.ref}
              className="react-player"
              width="100%"
              height="100%"
              url={url}
              pip={pip}
              playing={playing}
              controls={controls}
              light={light}
              loop={loop}
              playbackRate={playbackRate}
              volume={volume}
              muted={muted}
              onReady={() => console.log("onReady")}
              onStart={() => console.log("onStart")}
              onPlay={this.onPlay}
              onEnablePIP={this.onEnablePIP}
              onDisablePIP={this.onDisablePIP}
              onPause={this.onPause}
              onBuffer={() => console.log("onBuffer")}
              onSeek={e => console.log("onSeek", e)}
              onEnded={this.onEnded}
              onError={e => console.log("onError", e)}
              onProgress={this.onProgress}
              onDuration={this.onDuration}
            />
          </div>
          {this.renderMeta()}
          <button onClick={this.playPause}>
                    {playing ? "Pause" : "Play"}
          </button>
        </section>
      </div>
    );
  }
}

export default hot(module)(App);
