import './styles.css'
import { Render } from './hooks/render/render'
import { TitleBar } from './components/TitleBar/TitleBar'
import { BackgroundGradientAnimation } from './components/ui/BackgroundGradient/BackgroundGradient'
import { BackgroundBeams } from './components/ui/BackgroundBeams/BackgroundBeams'
import { Home } from './Pages/Home/Home'
import { ToastContainer } from 'react-toastify'
import 'react-toastify/dist/ReactToastify.css'

// eslint-disable-next-line react-refresh/only-export-components
const Main = (): JSX.Element => {
  return (
    <BackgroundGradientAnimation>
      <div className="h-screen absolute top-0 left-0 z-40 bg-black/25 w-screen rounded-lg overflow-hidden flex flex-col text-white">
        <TitleBar title="YouTube Downloader" />
        <Home />
      </div>
      <ToastContainer
        position="bottom-left"
        autoClose={1000}
        hideProgressBar
        newestOnTop={false}
        closeOnClick
        rtl={false}
        pauseOnFocusLoss
        draggable
        pauseOnHover
        theme="dark"
      />
      <BackgroundBeams />
    </BackgroundGradientAnimation>
  )
}

Render(<Main />)
