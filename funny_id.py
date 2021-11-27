"""
Generate human-friendly randomized ID strings.
"""

import random


def random_id(pattern: str = 'dan', separator: str = '_', rnd=random) -> str:
    """Generate a string by combining randomized words.

    Parameters
    ----------
    pattern : str, optional
        Specifies how to compose the id string. Each character in the pattern
        identifies a category and will be replaced by a word drawn randomly
        from that category. Supported categories are
          - 'd': determiners
          - 'a': adjectives
          - 'n': nouns
        The same category can occur multiple times in the pattern.
        The default is 'dan'.
    separator : str, optional
        String to put between individual words. The default is '_'.
    rnd : optional
        Random number generator to use. Can be any object that exposes a
        `choice` method. The default is Python's built-in `random` module.

    Returns
    -------
    str
        A randomized string. Sometimes it will be even funny.
    """
    source_map = {'d': DETERMINERS, 'a': ADJECTIVES, 'n': NOUNS}
    elements = (p for p in pattern)
    sources = map(source_map.get, elements)
    words = map(rnd.choice, sources)
    return separator.join(words)


def hash_id(hashnum: int, pattern: str = 'dan', separator: str = '_') -> str:
    source_map = {'d': DETERMINERS, 'a': ADJECTIVES, 'n': NOUNS}
    elements = (p for p in pattern)
    sources = list(map(source_map.get, elements))
    combinations = 1
    for s in sources:
        combinations *= len(s)

    idx = hashnum % combinations
    words = []
    for s in sources:
        words.append(s[idx % len(s)])
        idx = int(idx / len(s))

    return separator.join(words)


DETERMINERS = ['every', 'her', 'his', 'my', 'no', 'our', 'that', 'the', 'their', 'your']

ADJECTIVES = ['able', 'abnormal', 'above', 'absent', 'absolute', 'abstract', 'absurd', 'academic', 'acceptable',
              'accessible', 'accurate', 'active', 'actual', 'acute', 'additional', 'adequate', 'adjacent',
              'administrative', 'adverse', 'advisory', 'aesthetic', 'afraid', 'aggregate', 'aggressive', 'agricultural',
              'alien', 'alive', 'all', 'alone', 'alpine', 'alright', 'alternative', 'amateur', 'ambiguous', 'ambitious',
              'ample', 'ancient', 'angry', 'annual', 'anonymous', 'anxious', 'apparent', 'applicable', 'appropriate',
              'arbitrary', 'archaeological', 'architectural', 'arctic', 'artificial', 'artistic', 'ashamed', 'asleep',
              'assistant', 'atomic', 'attractive', 'authentic', 'automatic', 'autonomous', 'available', 'average',
              'awake', 'aware', 'awful', 'awkward', 'back', 'bad', 'ballistic', 'bare', 'basic', 'bass', 'beautiful',
              'behavioural', 'beneficial', 'bidirectional', 'big', 'bilateral', 'biological', 'bitter', 'bizarre',
              'black', 'blank', 'bleak', 'blind', 'blonde', 'bloody', 'blue', 'bodily', 'bold', 'bottom', 'bourgeois',
              'brave', 'brief', 'bright', 'brilliant', 'broad', 'brown', 'bureaucratic', 'busy', 'calm', 'capable',
              'capitalist', 'careful', 'casual', 'catholic', 'causal', 'cautious', 'central', 'certain',
              'characteristic', 'cheap', 'cheerful', 'chemical', 'chief', 'chronic', 'circular', 'civic', 'civil',
              'civilian', 'classic', 'classical', 'clean', 'clear', 'clerical', 'clever', 'clinical', 'close',
              'cooperative', 'coastal', 'cognitive', 'coherent', 'cold', 'collective', 'colonial', 'colonic',
              'colourful', 'comfortable', 'comic', 'commercial', 'common', 'communist', 'compact', 'comparable',
              'comparative', 'compatible', 'competent', 'competitive', 'complementary', 'complete', 'complex',
              'comprehensive', 'compulsory', 'conceptual', 'concrete', 'confident', 'confidential', 'conscious',
              'consequent', 'conservative', 'considerable', 'consistent', 'constant', 'constitutional', 'constructive',
              'contemporary', 'content', 'continental', 'continuous', 'contractual', 'contrary', 'controversial',
              'convenient', 'conventional', 'cool', 'corporate', 'correct', 'costly', 'crazy', 'creative', 'criminal',
              'critical', 'crucial', 'crude', 'cruel', 'cultural', 'curious', 'current', 'customary', 'daily', 'damp',
              'dangerous', 'dark', 'dead', 'deadly', 'deaf', 'dear', 'decent', 'decisive', 'decorative', 'deep',
              'defensive', 'definite', 'deliberate', 'delicate', 'delicious', 'delightful', 'democratic', 'dense',
              'departmental', 'dependent', 'desirable', 'desperate', 'destructive', 'different', 'difficult', 'digital',
              'diplomatic', 'direct', 'dirty', 'disastrous', 'disciplinary', 'distant', 'distinct', 'distinctive',
              'diverse', 'divine', 'divisional', 'domestic', 'dominant', 'double', 'doubtful', 'dramatic', 'dreadful',
              'dual', 'due', 'dull', 'dynamic', 'eager', 'early', 'eastern', 'easy', 'ecclesiastical', 'economic',
              'educational', 'effective', 'efficient', 'eighteenth', 'eighth', 'elaborate', 'elderly', 'electoral',
              'electric', 'electrical', 'electronic', 'elegant', 'eligible', 'emotional', 'empirical', 'empty',
              'endless', 'enjoyable', 'enormous', 'enthusiastic', 'entire', 'environmental', 'equal', 'equivalent',
              'essential', 'eternal', 'ethical', 'ethnic', 'eventual', 'everyday', 'evident', 'evil', 'evolutionary',
              'exact', 'excellent', 'exceptional', 'excess', 'excessive', 'exclusive', 'exotic', 'expensive',
              'experimental', 'explicit', 'extensive', 'external', 'extra', 'extraordinary', 'extreme', 'factual',
              'faint', 'fair', 'faithful', 'false', 'familiar', 'famous', 'fantastic', 'far', 'fashionable', 'fast',
              'fat', 'fatal', 'favourable', 'favourite', 'feasible', 'federal', 'fellow', 'female', 'feminine',
              'feminist', 'fierce', 'fifth', 'final', 'financial', 'fine', 'firm', 'first', 'fiscal', 'fit', 'flat',
              'flexible', 'fond', 'foolish', 'foreign', 'formal', 'former', 'formidable', 'forthcoming', 'fortunate',
              'forward', 'forward', 'fourth', 'fragile', 'free', 'frequent', 'fresh', 'friendly', 'front', 'full',
              'fulltime', 'fun', 'functional', 'fundamental', 'funny', 'furious', 'future', 'gastric', 'general',
              'generous', 'genetic', 'gentle', 'genuine', 'geographical', 'geological', 'german', 'giant', 'glad',
              'global', 'glorious', 'golden', 'good', 'gothic', 'gradual', 'grammatical', 'grand', 'grateful', 'grave',
              'great', 'green', 'grey', 'grim', 'gross', 'guilty', 'half', 'handsome', 'handy', 'happy', 'hard',
              'harmful', 'harsh', 'head', 'healthy', 'heavy', 'helpful', 'helpless', 'high', 'historic', 'historical',
              'holy', 'homeless', 'honest', 'horizontal', 'horrible', 'hostile', 'hot', 'huge', 'human', 'hungry',
              'ideal', 'identical', 'ideological', 'ill', 'illegal', 'imaginative', 'immediate', 'immense', 'imminent',
              'immune', 'imperial', 'implicit', 'important', 'impossible', 'impressive', 'inadequate', 'inappropriate',
              'incapable', 'incredible', 'independent', 'indigenous', 'indirect', 'individual', 'indoor', 'industrial',
              'inevitable', 'inferior', 'infinite', 'influential', 'informal', 'inherent', 'initial', 'inland', 'inner',
              'innocent', 'innovative', 'instant', 'institutional', 'instrumental', 'insufficient', 'intact',
              'integral', 'intellectual', 'intelligent', 'intense', 'intensive', 'interactive', 'interim', 'interior',
              'intermediate', 'internal', 'international', 'intestinal', 'intimate', 'invaluable', 'invisible',
              'irrelevant', 'irrespective', 'jealous', 'joint', 'judicial', 'junior', 'just', 'keen', 'key', 'kind',
              'large', 'last', 'late', 'latter', 'lazy', 'legal', 'legislative', 'legitimate', 'lengthy', 'lesser',
              'level', 'lexical', 'liable', 'liberal', 'light', 'like', 'likely', 'linear', 'linguistic', 'liquid',
              'literary', 'little', 'live', 'lively', 'local', 'logical', 'lone', 'lonely', 'long', 'longterm', 'loose',
              'loud', 'lovely', 'low', 'loyal', 'lucky', 'luxury', 'mad', 'magic', 'magical', 'magnetic', 'magnificent',
              'main', 'mainstream', 'major', 'male', 'managerial', 'mandatory', 'manual', 'marginal', 'marine',
              'marvellous', 'mass', 'massive', 'mathematical', 'mature', 'maximum', 'mean', 'meaningful', 'mechanical',
              'medical', 'medieval', 'medium', 'memorable', 'mental', 'mere', 'metropolitan', 'mid', 'middle',
              'middleclass', 'mighty', 'mild', 'military', 'miniature', 'minimal', 'minimum', 'ministerial', 'minor',
              'miserable', 'mobile', 'moderate', 'modern', 'modest', 'molecular', 'monetary', 'monthly', 'moral',
              'multiple', 'municipal', 'musical', 'mutual', 'mysterious', 'naked', 'narrow', 'nasty', 'national',
              'natural', 'naval', 'near', 'nearby', 'neat', 'necessary', 'negative', 'nervous', 'net', 'neutral', 'new',
              'next', 'nice', 'nineteenth', 'ninth', 'noble', 'noisy', 'nominal', 'normal', 'northern', 'notable',
              'noticeable', 'notorious', 'novel', 'nuclear', 'numerous', 'objective', 'obscure', 'obvious',
              'occasional', 'occupational', 'odd', 'oesophageal', 'offensive', 'official', 'okay', 'old',
              'oldfashioned', 'one', 'only', 'open', 'operational', 'opposite', 'optical', 'optimistic', 'optional',
              'oral', 'orange', 'ordinary', 'organic', 'organisational', 'organizational', 'original', 'orthodox',
              'other', 'outdoor', 'outer', 'outside', 'outstanding', 'overall', 'overseas', 'own', 'painful', 'pale',
              'papal', 'parallel', 'parental', 'parliamentary', 'parttime', 'partial', 'particular', 'passionate',
              'passive', 'past', 'patient', 'payable', 'peaceful', 'peculiar', 'perfect', 'peripheral', 'permanent',
              'persistent', 'personal', 'petty', 'philosophical', 'photographic', 'physical', 'pink', 'plain',
              'plausible', 'pleasant', 'polite', 'political', 'poor', 'popular', 'portable', 'positive', 'possible',
              'postwar', 'potent', 'potential', 'powerful', 'practical', 'pragmatic', 'pretax', 'precious', 'precise',
              'predictable', 'pregnant', 'preliminary', 'premature', 'premier', 'present', 'presidential', 'pretty',
              'previous', 'primary', 'prime', 'primitive', 'principal', 'prior', 'private', 'probabilistic', 'probable',
              'productive', 'professional', 'profitable', 'profound', 'progressive', 'prominent', 'prone', 'proper',
              'proportional', 'prospective', 'protective', 'protestant', 'proud', 'provincial', 'provisional',
              'psychiatric', 'psychological', 'public', 'pure', 'purple', 'quantitative', 'quick', 'quiet', 'racial',
              'radical', 'radioactive', 'random', 'rapid', 'rare', 'rational', 'raw', 'ready', 'real', 'realistic',
              'rear', 'reasonable', 'recent', 'red', 'redundant', 'regional', 'regular', 'regulatory', 'relative',
              'relevant', 'reliable', 'religious', 'reluctant', 'remarkable', 'remote', 'repeated', 'repeating',
              'repetitive', 'representative', 'resident', 'residential', 'respectable', 'respective', 'responsible',
              'restrictive', 'retail', 'retro', 'revolutionary', 'rich', 'ridiculous', 'right', 'rigid', 'rival',
              'roman', 'romantic', 'rotten', 'rough', 'round', 'royal', 'rubber', 'rude', 'rural', 'sacred', 'sad',
              'safe', 'same', 'satisfactory', 'scientific', 'seasonal', 'second', 'secondary', 'secret', 'secular',
              'secure', 'select', 'selective', 'semantic', 'senior', 'sensible', 'sensitive', 'separate', 'serious',
              'seventh', 'severe', 'shallow', 'sharp', 'sheer', 'short', 'shortterm', 'shy', 'sick', 'significant',
              'silent', 'silly', 'similar', 'simple', 'single', 'sixth', 'slight', 'slim', 'slow', 'small', 'smart',
              'smooth', 'socalled', 'social', 'socialist', 'sociological', 'soft', 'solar', 'sole', 'solid', 'solitary',
              'sore', 'sorry', 'sound', 'southern', 'soviet', 'spare', 'spatial', 'special', 'specific', 'spectacular',
              'spiritual', 'splendid', 'spontaneous', 'square', 'stable', 'standard', 'static', 'statistical',
              'statutory', 'steady', 'steep', 'sticky', 'stiff', 'still', 'straight', 'straightforward', 'strange',
              'strategic', 'strict', 'strong', 'structural', 'stupid', 'subject', 'subjective', 'subordinate',
              'subsequent', 'substantial', 'substantive', 'subtle', 'successful', 'successive', 'sudden', 'sufficient',
              'suitable', 'sunny', 'super', 'superb', 'superior', 'supplementary', 'supportive', 'supreme', 'sure',
              'surgical', 'surplus', 'suspicious', 'sweet', 'swift', 'symbolic', 'sympathetic', 'syntactic',
              'systematic', 'talented', 'tall', 'technical', 'technological', 'teenage', 'temporary', 'tender', 'tense',
              'tenth', 'terminal', 'terrible', 'territorial', 'then', 'theoretical', 'thick', 'thin', 'third',
              'thorough', 'tight', 'tiny', 'top', 'tory', 'total', 'tough', 'toxic', 'traditional', 'tragic',
              'tremendous', 'trivial', 'tropical', 'true', 'twelfth', 'twentieth', 'typical', 'ugly', 'ultimate',
              'unable', 'unacceptable', 'unaware', 'uncertain', 'unchanged', 'unclear', 'uncomfortable', 'unconscious',
              'underground', 'understandable', 'uneasy', 'unemployed', 'unexpected', 'unfair', 'unfamiliar',
              'unfortunate', 'unhappy', 'uniform', 'unique', 'universal', 'unknown', 'unlawful', 'unlikely',
              'unnecessary', 'unpleasant', 'unprecedented', 'unreasonable', 'unsatisfactory', 'unsuccessful', 'unusual',
              'unwanted', 'unwilling', 'upper', 'upset', 'urban', 'urgent', 'useful', 'useless', 'usual', 'vacant',
              'vague', 'vain', 'valid', 'valuable', 'variable', 'various', 'vast', 'verbal', 'vertical', 'very',
              'viable', 'vicious', 'vigorous', 'violent', 'virtual', 'visible', 'visual', 'vital', 'vivid',
              'vocational', 'voluntary', 'vulnerable', 'wannabe', 'warm', 'wary', 'weak', 'wealthy', 'wee', 'weekly',
              'weird', 'welcome', 'wellknown', 'western', 'wet', 'white', 'whole', 'wicked', 'wide', 'widespread',
              'wild', 'wise', 'wonderful', 'wooden', 'workingclass', 'worldwide', 'worthwhile', 'worthy', 'wrong',
              'yellow', 'young', 'zero']

NOUNS = ['ability', 'absence', 'abuse', 'access', 'accident', 'accommodation', 'account', 'acid', 'act', 'action',
         'activity', 'addition', 'address', 'administration', 'adult', 'advance', 'advantage', 'advice', 'afternoon',
         'age', 'agency', 'agent', 'agreement', 'agriculture', 'aid', 'aim', 'air', 'aircraft', 'alan', 'alternative',
         'amount', 'analysis', 'animal', 'answer', 'appeal', 'appearance', 'application', 'appointment', 'approach',
         'approval', 'area', 'argument', 'arm', 'army', 'arrival', 'art', 'article', 'artist', 'aspect', 'assembly',
         'assessment', 'assistance', 'association', 'atmosphere', 'attack', 'attempt', 'attention', 'attitude',
         'audience', 'author', 'authority', 'autumn', 'average', 'award', 'awareness', 'baby', 'back', 'background',
         'bag', 'balance', 'ball', 'band', 'bank', 'bar', 'base', 'basis', 'bath', 'battle', 'beach', 'beauty', 'bed',
         'bedroom', 'behalf', 'behaviour', 'belief', 'benefit', 'bill', 'bird', 'birth', 'bishop', 'block', 'blood',
         'board', 'boat', 'bob', 'body', 'book', 'border', 'bottle', 'bottom', 'box', 'boy', 'brain', 'branch', 'bread',
         'break', 'breakfast', 'breath', 'bridge', 'brother', 'brown', 'budget', 'bus', 'bush', 'business', 'cabinet',
         'call', 'campaign', 'cancer', 'candidate', 'capacity', 'capital', 'captain', 'car', 'card', 'care', 'career',
         'case', 'cash', 'castle', 'cat', 'cause', 'cell', 'centre', 'century', 'chain', 'chair', 'chairman',
         'challenge', 'championship', 'chance', 'chancellor', 'change', 'channel', 'chapter', 'character', 'charge',
         'charity', 'chest', 'chief', 'child', 'china', 'choice', 'christ', 'church', 'city', 'claim', 'class',
         'clause', 'client', 'club', 'co', 'cooperation', 'coal', 'coast', 'code', 'coffee', 'collection', 'college',
         'colour', 'combination', 'command', 'commission', 'commitment', 'committee', 'communication', 'community',
         'company', 'comparison', 'competition', 'computer', 'concentration', 'concept', 'concern', 'conclusion',
         'condition', 'conference', 'confidence', 'conflict', 'congress', 'connection', 'consequence', 'consideration',
         'constitution', 'construction', 'consumer', 'contact', 'content', 'context', 'contract', 'contrast',
         'contribution', 'control', 'convention', 'conversation', 'copy', 'core', 'corner', 'corporation', 'cost',
         'council', 'country', 'countryside', 'county', 'couple', 'course', 'court', 'cover', 'creation', 'credit',
         'crime', 'crisis', 'criticism', 'cross', 'crowd', 'crown', 'culture', 'cup', 'currency', 'curriculum',
         'customer', 'dad', 'damage', 'danger', 'date', 'daughter', 'day', 'deal', 'death', 'debate', 'debt', 'decade',
         'decision', 'decline', 'defence', 'definition', 'degree', 'delivery', 'demand', 'democracy', 'department',
         'deputy', 'description', 'design', 'desire', 'desk', 'detail', 'development', 'diet', 'difference',
         'difficulty', 'dinner', 'direction', 'director', 'discipline', 'discussion', 'disease', 'display', 'distance',
         'distinction', 'distribution', 'district', 'division', 'doctor', 'document', 'dog', 'door', 'doubt', 'drama',
         'dream', 'dress', 'drink', 'drive', 'driver', 'drug', 'duty', 'earth', 'east', 'economy', 'edge', 'editor',
         'education', 'effect', 'efficiency', 'effort', 'election', 'electricity', 'element', 'emergency', 'emphasis',
         'empire', 'employment', 'end', 'energy', 'engine', 'english', 'enterprise', 'entry', 'environment',
         'equipment', 'error', 'establishment', 'estate', 'event', 'evidence', 'examination', 'example', 'exchange',
         'executive', 'exercise', 'exhibition', 'existence', 'expansion', 'expenditure', 'experience', 'expert',
         'explanation', 'expression', 'extension', 'extent', 'eye', 'face', 'fact', 'factor', 'factory', 'failure',
         'faith', 'fall', 'family', 'farm', 'fashion', 'father', 'favour', 'fear', 'feature', 'field', 'fig.', 'figure',
         'file', 'film', 'finance', 'fire', 'firm', 'fish', 'flat', 'flight', 'floor', 'flow', 'focus', 'food', 'foot',
         'football', 'force', 'forest', 'form', 'formation', 'foundation', 'framework', 'frank', 'freedom', 'friend',
         'front', 'fruit', 'fuel', 'fun', 'function', 'fund', 'furniture', 'future', 'gallery', 'game', 'gap', 'garden',
         'gas', 'gate', 'general', 'generation', 'gentleman', 'girl', 'glass', 'goal', 'god', 'gold', 'golf',
         'government', 'graham', 'grant', 'grass', 'green', 'group', 'growth', 'guide', 'gun', 'guy', 'hair', 'half',
         'hall', 'hand', 'head', 'health', 'heart', 'heat', 'height', 'hell', 'help', 'henry', 'hill', 'history',
         'hole', 'holiday', 'home', 'hope', 'horse', 'hospital', 'hotel', 'hour', 'house', 'household', 'husband',
         'ice', 'idea', 'identity', 'image', 'impact', 'importance', 'impression', 'improvement', 'incident', 'income',
         'increase', 'independence', 'index', 'individual', 'industry', 'inflation', 'influence', 'information',
         'initiative', 'injury', 'inquiry', 'instance', 'institute', 'institution', 'insurance', 'intelligence',
         'intention', 'interest', 'interpretation', 'interview', 'introduction', 'investigation', 'investment',
         'involvement', 'iron', 'island', 'issue', 'item', 'jack', 'jane', 'japan', 'job', 'joe', 'john', 'jones',
         'journey', 'judge', 'justice', 'key', 'kind', 'king', 'kingdom', 'kitchen', 'knowledge', 'labour', 'lack',
         'lady', 'lake', 'land', 'lane', 'language', 'law', 'lead', 'leader', 'leadership', 'league', 'lee', 'leg',
         'legislation', 'length', 'letter', 'level', 'lewis', 'liability', 'library', 'licence', 'life', 'lifespan',
         'light', 'limit', 'line', 'link', 'list', 'literature', 'loan', 'location', 'look', 'lord', 'loss', 'lot',
         'love', 'lunch', 'machine', 'magazine', 'maintenance', 'major', 'majority', 'man', 'management', 'manager',
         'manchester', 'manner', 'map', 'march', 'mark', 'market', 'marriage', 'martin', 'mary', 'mass', 'master',
         'match', 'material', 'may', 'meal', 'measure', 'meat', 'member', 'membership', 'memory', 'message', 'metal',
         'method', 'middle', 'mike', 'milk', 'mill', 'mind', 'minister', 'ministry', 'minority', 'minute', 'mirror',
         'miss', 'mistake', 'model', 'moment', 'money', 'month', 'morning', 'mother', 'motion', 'motor', 'mountain',
         'mouth', 'move', 'movement', 'mum', 'murder', 'museum', 'music', 'name', 'nation', 'nature', 'neck', 'need',
         'network', 'new', 'news', 'newspaper', 'night', 'noise', 'north', 'northern', 'nose', 'note', 'notice',
         'notion', 'object', 'occasion', 'offence', 'offer', 'office', 'officer', 'oil', 'operation', 'opinion',
         'opportunity', 'opposition', 'option', 'order', 'organisation', 'organization', 'other', 'outcome', 'output',
         'owner', 'oxford', 'package', 'page', 'pain', 'pair', 'palace', 'panel', 'paper', 'parent', 'parish', 'park',
         'parliament', 'part', 'partner', 'partnership', 'party', 'passage', 'past', 'path', 'patient', 'pattern',
         'paul', 'pay', 'payment', 'peace', 'pension', 'performance', 'period', 'person', 'peter', 'phase',
         'philosophy', 'phone', 'picture', 'piece', 'place', 'plan', 'plane', 'plant', 'plastic', 'plate', 'play',
         'player', 'pleasure', 'point', 'poison', 'police', 'policy', 'pollution', 'pool', 'population', 'port',
         'position', 'possibility', 'post', 'potential', 'pound', 'power', 'practice', 'presence', 'present',
         'president', 'press', 'pressure', 'price', 'prince', 'principle', 'priority', 'prison', 'problem', 'procedure',
         'process', 'product', 'production', 'professor', 'profiler', 'profit', 'program', 'progress', 'project',
         'property', 'proportion', 'proposal', 'protection', 'provision', 'pub', 'public', 'publication', 'purpose',
         'quality', 'quarter', 'queen', 'question', 'race', 'radio', 'rail', 'railway', 'rain', 'range', 'rate',
         'reaction', 'reader', 'reality', 'reason', 'recession', 'recognition', 'record', 'recovery', 'reduction',
         'ref', 'reference', 'reform', 'regime', 'region', 'relation', 'relationship', 'release', 'relief', 'religion',
         'report', 'representation', 'republic', 'reputation', 'request', 'research', 'resistance', 'resolution',
         'respect', 'response', 'responsibility', 'rest', 'restaurant', 'result', 'retirement', 'return', 'revenue',
         'review', 'revolution', 'right', 'ring', 'rise', 'risk', 'river', 'road', 'rock', 'role', 'roof', 'room',
         'round', 'route', 'row', 'rugby', 'rule', 'run', 'safety', 'sale', 'sample', 'scale', 'scene', 'scheme',
         'school', 'science', 'scope', 'screen', 'sea', 'search', 'season', 'seat', 'second', 'secretary', 'section',
         'sector', 'security', 'selection', 'self', 'sense', 'sentence', 'sequence', 'series', 'server', 'service',
         'session', 'set', 'settlement', 'shape', 'share', 'sheet', 'ship', 'shock', 'shop', 'shoulder', 'show', 'side',
         'sight', 'sign', 'significance', 'silence', 'silver', 'simon', 'sir', 'sister', 'site', 'situation', 'size',
         'skill', 'skin', 'sky', 'sleep', 'smile', 'smith', 'snow', 'society', 'software', 'soil', 'solution', 'son',
         'song', 'sort', 'sound', 'source', 'south', 'space', 'speaker', 'specialist', 'species', 'speech', 'speed',
         'spirit', 'spokesman', 'sport', 'spot', 'spring', 'square', 'staff', 'stage', 'standard', 'star', 'start',
         'state', 'statement', 'station', 'status', 'steel', 'step', 'stock', 'stone', 'store', 'strategy', 'street',
         'strength', 'stress', 'strike', 'structure', 'struggle', 'student', 'study', 'stuff', 'style', 'subject',
         'success', 'sugar', 'sum', 'summer', 'sun', 'supply', 'support', 'surface', 'surprise', 'survey', 'system',
         'table', 'talk', 'tape', 'target', 'task', 'tax', 'taylor', 'tea', 'teacher', 'team', 'technique',
         'technology', 'telephone', 'television', 'temperature', 'term', 'test', 'text', 'thatcher', 'theatre', 'theme',
         'theory', 'thing', 'threat', 'time', 'title', 'tom', 'tone', 'tony', 'top', 'torch', 'total', 'touch', 'tour',
         'tower', 'town', 'track', 'trade', 'tradition', 'traffic', 'train', 'transfer', 'transport', 'travel',
         'treatment', 'treaty', 'tree', 'trial', 'trip', 'trouble', 'trust', 'truth', 'turn', 'tv', 'type', 'uncle',
         'unemployment', 'union', 'unit', 'university', 'us', 'use', 'user', 'valley', 'value', 'van', 'variety',
         'vehicle', 'version', 'victim', 'victory', 'video', 'view', 'village', 'violence', 'vision', 'visit', 'voice',
         'volume', 'vote', 'walk', 'wall', 'war', 'waste', 'water', 'way', 'wealth', 'weather', 'week', 'weekend',
         'weight', 'welfare', 'west', 'whale', 'while', 'white', 'whole', 'widow', 'wife', 'will', 'wind', 'window',
         'wine', 'winter', 'woman', 'wood', 'word', 'work', 'worker', 'world', 'writer', 'year', 'youth', 'zulu']
