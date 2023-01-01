link로 돼 있는 css랑 js를 파일에 직접 때려박기! dependency 없는 html로 만드는 거임!

css랑 js를 읽는 기능은 필요없음! css랑 js를 찾고 대체하는 건 engine이 할 거고 HXML은 관련 API만 적용해주면 됨 (css, js 찾기, 내용 읽기, 합치기 등등등...)

---

class랑 id로 검색할 일이 많잖아? 걔네를 위해서 table을 만들어둘까?? 그럼 검색이 O(n)에서 O(log m)이 되겠지 (n은 node의 개수 m은 table의 크기)