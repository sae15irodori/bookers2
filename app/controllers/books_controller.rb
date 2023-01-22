class BooksController < ApplicationController
  def index
    @books = Book.all
    @book = Book.new#入力ﾌｫｰﾑ(部分テンプレへ渡す)
    @user = current_user#部分テンプレへ渡す
  end

  def create
    @book = Book.new(book_params)
    @book.user_id = current_user.id#ﾌｫｰﾑに入力済ﾃﾞｰﾀのﾕｰｻﾞidにﾛｸﾞｲﾝ中のﾕｰｻﾞｰid代入
    if @book.save                 #ﾃﾞｰﾀ保存されたら
      flash[:notice] = "You have created book successfully."#ﾌﾗｯｼｭﾒｯｾｰｼﾞ
      redirect_to book_path(@book.id)#詳細ﾍﾟｰｼﾞへ飛ぶ
    else                        #ﾃﾞｰﾀ保存されなかったら
      @books = Book.all
      render :index#indexﾍﾟｰｼﾞを表示
    end
  end

  def show
    @book = Book.find(params[:id])#特定の投稿ﾃﾞｰﾀのid取得してレコードを取得
    @book_n = Book.new            #部分テンプレへ渡す
    @user = @book.user#部分テンプレへ渡す　投稿した人のﾕｰｻﾞ―情報
  end

  def destroy
    @book = Book.find(params[:id])
    @book.destroy
    redirect_to books_path
  end

  def edit
    @book = Book.find(params[:id])
  end

  def update
    @book = Book.find(params[:id])
    if @book.update(book_params)     #データ保存されたら
      flash[:notice] = "You have updated book successfully."
      redirect_to book_path(@book.id)
    else                             #データ保存されなかったら
      render :edit
    end
  end

  private
    def book_params
      params.require(:book).permit(:title, :body,)
    end

end
