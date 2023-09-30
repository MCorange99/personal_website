import { createSignal, type Component } from 'solid-js';
import { A, Navigate, Route, Router, Routes } from '@solidjs/router';

import Header from "./components/Header";
import Home from './components/Home';
import Blog from './components/Blog';
import Socials from './components/Socials';
import About from './components/About';
import Donate from "./components/Donate"
import Redirect from './components/Redirect';


const App: Component = () => {
    return (
        <>
            <Router> 
                <Header/>
                <div class="main-content">
                    <Routes>
                        <Route path="/" component={Home} />
                        <Route path="/blog" component={Blog} />
                        <Route path="/social" component={Socials} />
                        <Route path="/about" component={About} />
                        <Route path="/donate" component={Donate} />
                        <Route path="/r/:id" component={Redirect}>
                            {/* {redirects.map((val)=>{
                                return (
                                    <Route path={val.href} component={()=>{
                                        window.location.assign(val.url);
                                        return (<></>);
                                    }}/>
                                )
                            })} */}
                        </Route>
                    </Routes>
                </div>
            </Router>
        </>
    );
};

export default App;