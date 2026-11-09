"""Native bitmap font conversion preserves glyph shapes and proportional metrics."""
import io,sys,unittest
from pathlib import Path
from PIL import Image
from fontTools.ttLib import TTFont
from fontTools.pens.areaPen import AreaPen
sys.path.insert(0,str(Path(__file__).resolve().parents[1]/'tools'))
from game_ui import font_bytes
class NativeFontTest(unittest.TestCase):
 def test_holes_and_proportional_advance_survive_conversion(self):
  ring=Image.new('L',(3,3),255);ring.putpixel((1,1),0)
  font=TTFont(io.BytesIO(font_bytes({ord('I'):(ring,4),ord('W'):(ring,8)})))
  cmap=font.getBestCmap();self.assertEqual(font['hmtx'].metrics[cmap[ord('I')]][0],4*64)
  self.assertEqual(font['hmtx'].metrics[cmap[ord('W')]][0],8*64)
  pen=AreaPen(font.getGlyphSet());font.getGlyphSet()[cmap[ord('I')]].draw(pen)
  self.assertEqual(abs(pen.value),8*64*64)
