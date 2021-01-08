#Virtual dom 
O virtual dom foi criado para tornar a renderização no Real Dom mais performática. Ele atua em 3 etapas:  

Na primeira ele cria o Real dom, depois ele faz uma cópia virtual dele e por último ele faz faz as atualizações.  
Ou seja, ele tem em memória uma cópia virtual do Real Dom, e a partir daí ele identifica as alterações que ocorrem em layout, css ou ações e atualiza somente o que foi alterado
e essa alteração por sua vez não gera uma re-rendirizalção no Real Dom, pois o Virtual Dom processará essa mudança e irá refletir ela no Real Dom. 
