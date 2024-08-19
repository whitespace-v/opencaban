import Container from "./Components/HOC/Container";
import Float from "./Components/HOC/Float";
import HomeContentplates from "./Pages/Home/HomeContentplates";
import HomeIntro from './Pages/Home/HomeIntro'

export default function Home() {
  return (
    <div>
      <Container>
        <Float />
        <HomeIntro />
        <HomeContentplates />
      </Container>
    </div>
  );
}
